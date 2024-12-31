APPNAME=rust-app
REBAR=`which cargo || echo ./cargo`
REBAR=`which trunk || echo ./trunk`

all: deps compile

start:
	cd backend && cargo run &
	cd frontend && trunk serve
