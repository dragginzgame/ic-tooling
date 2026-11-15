#!/usr/bin/env bash
# Shared by active consumer runners. Caller supplies source_checkout and scratch.
# Copy current local inputs without writing to the sibling or exporting ignored tools.
export_local_source() {
  local pass name head_before head_after
  source_checkout="$(cd "$source_checkout" && pwd -P)" || return 1
  head_before="$(git -C "$source_checkout" rev-parse HEAD)" || return 1
  for pass in before after; do
    git -C "$source_checkout" ls-files --cached --others --exclude-standard --deduplicate -z > "$scratch/list-$pass.raw" || return 1
    : > "$scratch/list-$pass"
    while IFS= read -r -d '' name; do
      if [[ -L "$source_checkout/$name" || -f "$source_checkout/$name" ]]; then
        printf '%s\0' "$name" >> "$scratch/list-$pass"
      elif [[ -e "$source_checkout/$name" ]]; then
        printf 'Unsupported source entry (for example a submodule): %s\n' "$name" >&2
        return 1
      fi
    done < "$scratch/list-$pass.raw"
    LC_ALL=C sort -zu "$scratch/list-$pass" -o "$scratch/list-$pass" || return 1
    tar -C "$source_checkout" --null --verbatim-files-from -T "$scratch/list-$pass" -cf "$scratch/source-$pass.tar" || return 1
  done
  head_after="$(git -C "$source_checkout" rev-parse HEAD)" || return 1
  [[ "$head_before" == "$head_after" ]] &&
    cmp "$scratch/list-before" "$scratch/list-after" &&
    cmp "$scratch/source-before.tar" "$scratch/source-after.tar" || {
      printf 'Local source changed during snapshot; retry once editing has settled.\n' >&2
      return 1
    }
  printf '%s\n' "$source_checkout" > "$scratch/source-checkout"
  printf '%s\n' "$head_before" > "$scratch/source-head"
  sha256sum "$scratch/source-before.tar" > "$scratch/source.sha256" || return 1
  tar -xf "$scratch/source-before.tar" -C "$scratch/source" || return 1
  rm "$scratch/source-after.tar" "$scratch/list-before.raw" "$scratch/list-after.raw"
}
