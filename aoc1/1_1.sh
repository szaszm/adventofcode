#!/usr/bin/env bash

#sum=0

lines_to_numbers() {
	while read line; do
		[ -z "$line" ] && continue
		number="$(echo "$line$line"|sed -r -e 's/^[a-zA-Z]*([0-9])/\1/' -e 's/([0-9])[a-zA-Z]*$/\1/' -e 's/^([0-9]).*([0-9])$/\1\2/')"
		echo $number + 
		echo $number + 1>&2
	done
}

echo `lines_to_numbers` | sed -r -e 's/\+*$//g' -e 's/ //g' | bc
