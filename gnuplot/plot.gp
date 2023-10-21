set title filename
set xlabel "X (m)"
set ylabel "Zeta (m)"
set xrange [:250000]

plot filename \
        using 1:2 with lines title 'y=0km', \
     '' using 1:3 with lines title 'y=-100km', \
     '' using 1:4 with lines title 'y=-200km', \
     '' using 1:5 with lines title 'y=-300km', \
     '' using 1:6 with lines title 'y=-400km', \
     '' using 1:7 with lines title 'y=-500km', \
