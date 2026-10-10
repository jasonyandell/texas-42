CXX = clang++
PYTHON ?= python3
CXXFLAGS = -O3 -std=c++17 -fPIC -Wall -Wextra -Werror

.PHONY: all verify verify-addressed verify-patterns sanitize sanitize-thread
all: turbo.dylib

turbo.dylib: turbo.cpp
	$(CXX) $(CXXFLAGS) -shared $< -o $@

verify: turbo.dylib
	$(PYTHON) verify_turbo.py

verify-addressed: turbo.dylib
	$(PYTHON) verify_amortize.py

verify-patterns:
	$(CXX) $(CXXFLAGS) verify_patterns.cpp -o /tmp/walt42x-verify-patterns
	/tmp/walt42x-verify-patterns

sanitize:
	$(CXX) -O1 -g -std=c++17 -fsanitize=address,undefined -fno-omit-frame-pointer turbo_sanitize.cpp -o /tmp/walt42x-turbo-sanitize
	/tmp/walt42x-turbo-sanitize

sanitize-thread:
	$(CXX) -O1 -g -std=c++17 -fsanitize=thread turbo_thread_sanitize.cpp -o /tmp/walt42x-thread-sanitize
	/tmp/walt42x-thread-sanitize
