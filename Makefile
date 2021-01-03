all: rusthello rusthello-wasm rusthello-web
	$(info let's deploy rusthello-web/dist as a static web site)

rusthello: 
	cd rusthello && make

rusthello-wasm:
	cd rusthello-wasm && make

rusthello-web:
	cd rusthello-web && make

.PHONY: all rusthello rusthello-wasm rusthello-web