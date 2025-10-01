import time

start = time.time()
i = 0
while i < 1000000:
    i = i + 1
print(i)
end = time.time()
print(f"Python took: {end - start:.2f} seconds")
