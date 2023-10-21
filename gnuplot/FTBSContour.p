set contour
unset surface
set pm3d map
set view map
set dgrid3d 40 40
set key outside
set cntrparam level incremental -3, 0.03, 3
splot 'case2_FTBS.txt' with lines
