# Copyright 2014 The GL-RS Developers. For a full listing of the authors,
# refer to the AUTHORS file at the top-level directory of this distribution.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

RUSTC               = rustc
RUSTDOC             = rustdoc

SRC_DIR             = src
TEST_DIR            = test
LIB_FILE            = $(SRC_DIR)/gl/lib.rs
GEN_FILE            = $(SRC_DIR)/gen/main.rs
EXAMPLE_FILES       = $(SRC_DIR)/examples/*.rs

BIN_DIR             = bin
DEPS_DIR            = deps
DOC_DIR             = doc
EXAMPLES_DIR        = examples
LIB_DIR             = lib

GLRSGEN             = $(BIN_DIR)/glrsgen

GL_NAMESPACE        ?= gl
GL_API              ?= gl
GL_PROFILE          ?= core
GL_VERSION          ?= 4.3
GL_EXTENSIONS       ?=
GL_FULL             ?=
GL_XML              ?= $(DEPS_DIR)/khronos-api/$(GL_API).xml

GEN_FLAGS           ?= \
	--namespace $(GL_NAMESPACE) \
	--api $(GL_API) \
	--profile $(GL_PROFILE) \
	--version $(GL_VERSION) --xml $(GL_XML) \
	$(foreach EXTENSION, $(GL_EXTENSIONS), --extension $(EXTENSION)) \
	$(if $(GL_FULL), --full)

all: lib examples doc

check:
	@mkdir -p $(TEST_DIR)
	$(RUSTC) --out-dir=$(TEST_DIR) --test $(LIB_FILE)
	$(TEST_DIR)/gl

doc:
	@mkdir -p $(DOC_DIR)
	$(RUSTDOC) -o $(DOC_DIR) $(LIB_FILE)

examples-dir:
	mkdir -p $(EXAMPLES_DIR)

examples-deps:
	make lib -C $(DEPS_DIR)/glfw-rs

$(EXAMPLE_FILES): lib examples-dir examples-deps
	$(RUSTC) -L $(DEPS_DIR)/glfw-rs/lib -L $(LIB_DIR) --out-dir=$(EXAMPLES_DIR) $@

examples: $(EXAMPLE_FILES)

gen-deps:
	@git submodule init
	@git submodule update
	make lib -C $(DEPS_DIR)/sax-rs

gen: gen-deps
	@mkdir -p $(BIN_DIR)
	$(RUSTC) -L $(DEPS_DIR)/sax-rs/lib $(GEN_FILE) -o $(GLRSGEN)

gen-lib: gen
	@mkdir -p $(LIB_DIR)
	$(GLRSGEN) $(GEN_FLAGS) > $(LIB_FILE)

lib: gen-lib
	$(RUSTC) $(LIB_FILE) -O --out-dir=$(LIB_DIR)

clean:
	rm -f $(LIB_FILE)
	rm -rf $(BIN_DIR)
	rm -rf $(TEST_DIR)
	rm -rf $(DOC_DIR)
	make clean -C $(DEPS_DIR)/glfw-rs
	make clean -C $(DEPS_DIR)/sax-rs

.PHONY: \
	all \
	lib \
	check \
	doc \
	examples \
	examples-deps\
	examples-dir \
	gen \
	gen-deps \
	gen-lib \
	$(EXAMPLE_FILES) \
	clean
