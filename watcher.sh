#! /usr/bin/env zsh


export RUST_BACKTRACE=0;
export RUST_LOG=debug,test=debug;
# export RUSTFLAGS="-Z macro-backtrace -Z debug-macros"

function rebuild_invoicer {
  echo "\n\n\n\t\t<---------------------->\n\nBuilding and running the full test\n"
  cargo run
  echo "\n\n\n"
}

function init {
  echo "Running initialization"
  # echo "Running docker compose initialization"
  # make dockerDev
  cargo build
}


# Remove all the docker containers before exiting
function tearDown {
  echo "All done, tearing down"
  #/usr/bin/docker-compose -f scripts/docker/dev.docker-compose.yml down
}


# Initialize items like docker compose
init
space=" "
modify="${space}MODIFY${space}"

# And run it the first time before the loop so we don't have to wait for the update
rebuild_invoicer

while true; do
  command -v inotifywait > /dev/null 2>&1 || $(echo -e "InotifyWait not installed" && exit 1)
  echo -e $(pwd)
  EVENT=$(inotifywait -r -e modify \
    ./watcher.sh \
    ./Cargo.toml \
    ./the_process_foundry/Cargo.toml \
    ./the_process_foundry/src \
    ./the_process_foundry/book \
  )
  FILE_PATH=${EVENT/${modify}/}
  # echo -e "\nReceived event on file: '${FILE_PATH}'"

  # Root cases
  if [[ $FILE_PATH =~ "watcher.sh" ]]; then
    echo "Matched Watcher.sh. Exiting so we can restart"
    tearDown
    sleep 1
    exit 0

  elif [[ $FILE_PATH =~ "./Cargo.toml$" ]]; then
    rebuild_invoicer

  elif [[ $FILE_PATH =~ "^..?/.+.rs$" ]]; then
    rebuild_invoicer

  elif [[ $FILE_PATH =~ "^./.+.xlsx$" ]]; then
    rebuild_invoicer

  elif [[ $FILE_PATH =~ "^./.+.sql$" ]]; then
    rebuild_invoicer

  else
    echo -en "No Match on '${FILE_PATH}'': Continuing"

  fi
done
