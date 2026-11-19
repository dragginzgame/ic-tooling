#!/usr/bin/env bash
# The caller supplies Cargo supplier overrides (patches) and its evidence directory.
run_cases() {
  local package=$1 target=$2 selector name line count
  shift 2
  local -a names=() command=(cargo test "${patches[@]}" --locked -p "$package" "$target" --)
  "${command[@]}" --list --color never > "$scratch/$package.list"
  for selector do
    count=${#names[@]}
    while IFS= read -r line; do
      [[ $line == *': test' ]] || continue
      name=${line%': test'}
      if [[ $name == "$selector" || ( $selector == *:: && $name == "$selector"* ) ]]; then
        names+=("$name")
      fi
    done < "$scratch/$package.list"
    if [[ ${#names[@]} == "$count" ]]; then
      echo "Required Canic test selector missing from Cargo listing: $selector" >&2
      return 1
    fi
  done
  "${command[@]}" --exact --color never "${names[@]}" | tee "$scratch/$package.log"
  for name in "${names[@]}"; do
    grep -Fxq "test $name ... ok" "$scratch/$package.log" || {
      echo "Required Canic test did not pass: $name" >&2; return 1
    }
  done
}
