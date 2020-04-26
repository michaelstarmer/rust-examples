#!/bin/bash

set -e

KRED="\x1B[31m"   
KNRM="\x1B[0m"                    
KGRN="\x1B[32m"

function ERROR() {
                  
    printf $KRED"[ ERROR ]: "$KNRM
    printf "%s " "$@"             
    printf "\n"                   
    exit 1                        
}                                 
                                  
function OK() {      
               
    printf $KGRN"[ OK ]: "$KNRM
    printf "%s " "$@"          
    printf "\n"                
}

function SUCCESS() {
    printf $KGRN"[ SUCCESS ]: "$KNRM
    printf "%s " "$@"
    printf "\n\n"
}

function createProject() {
  OK "Generating project directory" && { cp -r ./_template ./$TARGET && cd ./$TARGET && sleep 1 ;}
  OK "Deleting old build files" && { rm -rf *.lock target ;}
  OK "Updating project title" && { sed -i "s/template/$TARGET/g" ./Cargo.toml && sed -i "s/\$project\-title/$TARGET/g" ./src/main.rs ;}
  
  SUCCESS "Project created: '$TARGET'"
}

function buildProject() {
  OK "Running build. This may take a minute..." && { set +x ; cargo build ;} 2>/dev/null
  OK "Done! Cleaning up..." && { sleep 2 ;}
  SUCCESS "Project build complete"
}

TARGET=$1

if [ $# -eq 0 ]; then
  echo "Parameter missing: New project name"
  exit 1
fi


createProject
buildProject

exit 0
