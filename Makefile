# UTC time in miliseconds
NOW:=`date -u +%s000`

NAME:=The Order of Mobius
SYSTEM:=Exioce

all:
	echo $(NOW)

update: clean fetch

clean:
	rm -f history.json

fetch: history.json

history.json:
	# get last 7 days worth of history
	# and format cleanly
	wget -O - "https://elitebgs.kodeblox.com/api/ebgs/v4/factions?name=$(NAME)&timemax=$(NOW)" | json_xs -f json -t json-pretty > history.json
