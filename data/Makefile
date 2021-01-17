BDF_DIR = ./fonts
OUT_DIR = ./fonts

# Match all files; just in case (tm).
# BDF_FILES = $(wildcard $(BDF_DIR)/*.bdf)
# OUT_FILES = $(patsubst $(BDF_DIR)/%.bdf,$(OUT_DIR)/%.rawfont,$(BDF_FILES))

PYTHON = python3
CONVERT_SCRIPT = ./gen_rawfont.py

NORMAL_RAWFONTS :=
REDUCED_RAWFONTS := ArialMT-24 TinyPixel-5 

NORMAL_RAWFONT_FILES = $(patsubst %,$(OUT_DIR)/%.rawfont,$(NORMAL_RAWFONTS))
REDUCED_RAWFONT_FILES = $(patsubst %,$(OUT_DIR)/%-Reduced.rawfont,$(REDUCED_RAWFONTS))

all: .pip.lock $(NORMAL_RAWFONTS_FILES) $(REDUCED_RAWFONT_FILES)

$(OUT_DIR)/%-Reduced.rawfont: $(BDF_DIR)/%.bdf
	$(PYTHON) $(CONVERT_SCRIPT) --reduced $<

$(OUT_DIR)/%.rawfont: $(BDF_DIR)/%.bdf
	$(PYTHON) $(CONVERT_SCRIPT) $<

.pip.lock: requirements.txt
	$(PYTHON) -m pip install -r requirements.txt
	touch .pip.lock

requirements.txt:
	touch requirements.txt

clean:
	rm -f $(OUT_DIR)/*.rawfont
	rm -f $(OUT_DIR)/*.rawfontc
	rm -f $(OUT_DIR)/*.rftable

reset: clean
	rm -f .pip.lock
