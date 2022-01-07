#!/bin/bash -e

if [ -z "$1" ] || [[ $1 != day* ]]; then
  echo "Usage: $0 <day>"
  exit 1
fi

DAY=$1
ROOT="."

for f in main.rs lib.rs; do
  if [ ! -f $ROOT/src/$f ]; then
    echo "$f file is missing"
    exit 1
  fi
done

if [ -e $ROOT/src/$DAY ] || grep -q "pub mod $DAY;" $ROOT/src/lib.rs; then
  echo "Day $DAY already exists"
  exit 1
fi

mkdir $ROOT/src/$DAY
sed -i "s/day[0-9][0-9]/$DAY/" $ROOT/src/main.rs
echo "pub mod $DAY;" >> $ROOT/src/lib.rs
cat > src/$1/mod.rs <<TEMPLATE
use super::common::Step;

pub fn test_result(step: &Step) -> String {
    match step {
	Step::First => 0.to_string(),
	Step::Second => 0.to_string(),
    }
}

fn count(_: &[&String]) -> i32 {
    0
}

pub fn solution(step: &Step, input: &[String]) -> String {
    let data:Vec<&String> = input
        .iter()
        .collect();
    match step {
        Step::First => count(&data).to_string(),
        Step::Second => count(&data).to_string(),
    }
}
TEMPLATE

