#!/bin/bash

#set -e

KRED="\x1B[31m"
KNRM="\x1B[0m"
KGRN="\x1B[32m"
KCYN="\x1b[36m"

IS_DEBUG = false
PROJECT_TITLE=$1
PROJECT_PATH="/home/michael/code/projects/rust/projects/small-and-simple/src/$PROJECT_TITLE"
PROJECT_PATH_RELATIVE="./src/$PROJECT_TITLE"

function ERROR() {
    printf "\n"
    printf $KRED"[ ERROR ]: "$KNRM
    printf "%s " "$@"             
    printf "\n\n"
    exit 1
}                                 
                                  
function OK() {      
               
    printf $KGRN"[ OK ]: "$KNRM
    printf "%s " "$@"          
    printf "\n"
    sleep 1
}

function SUCCESS() {
    printf "\n"
    printf $KGRN"[ SUCCESS ]: "$KNRM
    printf "%s " "$@"
    printf "\n"
}

function CODE() {
  printf $KCYN"$@"$KNRM
}

function createProject() {

  error="$(cd $PROJECT_PATH 2>&1)"

  [[ -z $error ]] && { ERROR "Project with title $PROJECT_NAME already exists." ;}
  
  OK "Generating project directory" && { cp -r ./_template $PROJECT_PATH && cd $PROJECT_PATH ;}

  OK "Deleting old build files" && { rm -rf *.lock target ;}
  OK "Updating project title" && { sed -i "s/template/$PROJECT_TITLE/g" $PROJECT_PATH/Cargo.toml && sed -i "s/\$project\-title/$PROJECT_TITLE/g" $PROJECT_PATH/src/main.rs ;}
  
  OK "Project created: '$PROJECT_PATH'"
}

function buildProject() {

  if [ "$IS_DEBUG" = false ] ; then
    OK "Running build. This may take a minute..." && \
    { cargo build 2>/dev/null ;}
  else
    OK "DEBUG MODE: Skipping build." ;
  fi

  OK "Done! Cleaning up..."
  SUCCESS "Project build complete"
}

function input() {
  longopts="help,debug::"
  args=$(getopt -o h --longoptions ${longOpts} -- "$@")

  while true ; do
    case "$2" in
      --debug )
        IS_DEBUG=true ; shift 2 ;;
      -- ) shift; break ;;
      *  ) break ;;
    esac
  done

  IS_DEBUG=${IS_DEBUG:-false}
}

if [ $# -eq 0 ]; then
  echo "Parameter missing: New project name"
  exit 1
fi

clear

input "$@"
createProject
buildProject

printf "\n 1. Go to project\n\n"
CODE "\tcd $PROJECT_PATH_RELATIVE \n"
printf "\n 2. Start\n"
printf "\n"
CODE "\tRUST_LOG=trace cargo run\n\n"

exit 0
