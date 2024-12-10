import time
import sys
try:
	cookTimeSeconds = int(sys.argv[1])
except ValueError:
	cookTimeSeconds = 0
cookTimeMinutes = cookTimeSeconds / 60
cookTimeMinutesDisplay = int(cookTimeMinutes)
cookTimeSecondsDisplay = cookTimeSeconds % 60
frequencyHertz = 10
sleepTime = 1.0 / frequencyHertz
repetitionCount = cookTimeSeconds * frequencyHertz
print("COOK TIME: %02d:%02d" % (cookTimeMinutesDisplay, cookTimeSecondsDisplay))
print("BEEP!")
for i in range(repetitionCount):
	print("m", end='', flush=True)
	time.sleep(sleepTime)
print()
print("BEEP! BEEP! BEEP!")
print("Food's done! :)")
