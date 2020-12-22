BDF_DIR = ./fonts
RAW_DIR = ./fonts
BDF_FILES = $(wildcard $(BDF_DIR)/*.bdf)
RAW_FILES = $(patsubst $(BDF_DIR)/%.bdf,$(RAW_DIR)/%.rawfont,$(BDF_FILES))

PYTHON = python3
CONVERT_SCRIPT = ./gen_rawfont.py

all: .pylock $(RAW_FILES)

$(RAW_DIR)/%.rawfont: $(BDF_DIR)/%.bdf
	$(PYTHON) $(CONVERT_SCRIPT) -o $@ --reduced $<

.pylock: requirements.txt
	$(PYTHON) -m pip install -r requirements.txt
	touch .pylock

requirements.txt:
	touch requirements.txt

clean:
	rm -f $(RAW_DIR)/*.rawfont
	rm -f $(RAW_DIR)/*.rawfontc
	rm -f $(RAW_DIR)/*.rftable

reset: clean
	rm -f .pylock
