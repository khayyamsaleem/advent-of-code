source .env

INPUT=$(echo $(http --no-verify https://adventofcode.com/2019/day/9/input Cookie:session=$session | xargs))

echo "PART ONE"
http POST intcode.docker.localhost/eval program:="[${INPUT}]" inputs:='[1]' program_counter:=0

echo "PART TWO"
http POST intcode.docker.localhost/eval program:="[${INPUT}]" inputs:='[2]' program_counter:=0
