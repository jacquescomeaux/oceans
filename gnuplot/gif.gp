set terminal gif animate delay 50
set output 'solution.gif'

stats 'solution.txt' nooutput

set pm3d map
set cbrange [0:1000]
set palette rgbformulae 23, 3, 28

set xlabel 'X (km)'
set ylabel 'Y (km)'

do for [i=1:int(STATS_blocks)] {
  splot 'solution.txt' index (i-1)
}
