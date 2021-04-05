#!/bin/bash

# IFS=

# while true
# do
# 	read -n 1 key
# 	if [ "$key" = "" ]; then
# 		cd /home/tharindu/Projects/rust/my-tui-app/target/debug
# 		# exec ./my-tui-app
# 		PROMPT_COMMAND="$PROMPT_COMMAND"$'\n'"exec ./my-tui-app"
# 	fi
# done

PROMPT_COMMAND=${PROMPT_COMMAND:+"$PROMPT_COMMAND; "}'. /home/tharindu/Projects/rust/my-tui-app/target/debug/my-tui-app'