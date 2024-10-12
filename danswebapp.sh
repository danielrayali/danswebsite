#!/bin/bash
# chkconfig: 2345 20 80
# description: Starts and stops dans-web-app

# Source function library.
# . /etc/init.d/functions

start() {
    start-stop-daemon --start --background --exec /work/dans-web-app --pidfile /work/danswebapp.pid --chdir /work --name danswebengine
}

stop() {
    # code to stop app comes here
    # example: killproc program_name
    start-stop-daemon --stop --pidfile /work/danswebapp.pid --name danswebengine
}

case "$1" in
    start)
       start
       ;;
    stop)
       stop
       ;;
    restart)
       stop
       start
       ;;
    status)
       # code to check status of app comes here
       # example: status program_name
       ;;
    *)
       echo "Usage: $0 {start|stop|status|restart}"
esac

exit 0