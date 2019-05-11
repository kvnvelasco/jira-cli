#/usr/bin/env bash

_complete_jira_issues() {
  echo 0 $0 >> completion_data
  echo 1 $1 >> completion_data
  echo 2 $2 >> completion_data
  echo 3 $3 >> completion_data
  echo 4 $4 >> completion_data
  echo $COMP_WORDS >> completion_data

  if [ "$3" = "jira" ]; then 
    COMPREPLY=($(compgen -W "$(work list-contexts -c jira)" "${COMP_WORDS[1]}"))
  fi
}

complete -F _complete_jira_issues work