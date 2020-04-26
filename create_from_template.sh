#!/bin/bash

#set -e

KRED="\x1B[31m"
KNRM="\x1B[0m"
KGRN="\x1B[32m"

PROJECT_TITLE=$1
PROJECT_PATH="/home/michael/code/projects/rust/projects/small-and-simple/src/$PROJECT_TITLE"

function ERROR() {
    printf "\n\n"
    printf $KRED"[ ERROR ]: "$KNRM
    printf "%s " "$@"             
    printf "\n\n"
    exit 1
}                                 
                                  
function OK() {      
               
    printf $KGRN"[ OK ]: "$KNRM
    printf "%s " "$@"          
    printf "\n"
    sleep 2
}

function SUCCESS() {
    printf $KGRN"[ SUCCESS ]: "$KNRM
    printf "%s " "$@"
    printf "\n\n"
}

function createProject() {

  error="$(cd $PROJECT_PATH 2>&1)"
#  echo "TEST" && { if test -z $error ; then echo "IS EMPTY: TRUE" ; else ERROR "Project with name $PROJECT_NAME exists. E: $error" ; fi ;}
  
  [[ -z $error ]] && { ERROR "Project with title $PROJECT_NAME already exists." ;}
  
  OK "Generating project directory" && { cp -r ./_template $PROJECT_PATH && cd $PROJECT_PATH ;}

  OK "Deleting old build files" && { rm -rf *.lock target ;}
  OK "Updating project title" && { sed -i "s/template/$PROJECT_TITLE/g" $PROJECT_PATH/Cargo.toml && sed -i "s/\$project\-title/$PROJECT_TITLE/g" $PROJECT_PATH/src/main.rs ;}
  
  SUCCESS "Project created: '$PROJECT_PATH'"
}

function buildProject() {
  OK "Running build. This may take a minute..." && { set +x ; cargo build ;} 2>/dev/null
  OK "Done! Cleaning up..."
  SUCCESS "Project build complete"
}

if [ $# -eq 0 ]; then
  echo "Parameter missing: New project name"
  exit 1
fi


createProject
buildProject

exit 0
