# source: https://www.geeksforgeeks.org/dsa/estimating-value-pi-using-monte-carlo/

import random
from math import pi as PI

INTERVAL = 100_000
LIMITS = (-1, 1)

circle_points = 0
square_points = 0

for i in range(INTERVAL):
	
	rand_x = random.uniform(*LIMITS)
	rand_y = random.uniform(*LIMITS)
	
	origin_dist = rand_x ** 2 + rand_y ** 2
	if origin_dist <= 1.0:
		circle_points += 1
	square_points += 1
	
	pi_appro = 4 * circle_points / square_points
	pi_delta = abs(PI - pi_appro)
	
	if i % 10_000 == 0:
		print(f"{i = }\t{pi_appro = }\t{pi_delta = }")

print(f"{i = }\t{pi_appro = }\t{pi_delta = }")
