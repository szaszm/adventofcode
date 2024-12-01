#!/usr/bin/env bash

#sum=0

lines_to_numbers() {
	while read line; do
		[ -z "$line" ] && continue
		number="$(echo "a${line}${line}a"|sed -r -e 's/one/1/g' -e 's/two/2/g' -e 's/three/3/g' -e 's/four/4/g' -e 's/five/5/g' -e 's/six/6/g' -e 's/seven/7/g' -e 's/eight/8/g' -e 's/nine/9/g' -e 's/zero/0/g' | sed -r -e 's/^[a-zA-Z]*([0-9])/\1/' -e 's/([0-9])[a-zA-Z]*$/\1/' -e 's/^([0-9]).*([0-9])$/\1\2/')"
		echo $number + 
		echo $number + 1>&2
	done
}

echo `lines_to_numbers` | sed -r -e 's/\+*$//g' -e 's/ //g' | bc
