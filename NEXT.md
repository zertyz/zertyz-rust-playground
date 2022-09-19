Crates to investigate:
	TIME MEASUREMENTS:                          minstant,quanta
	DB:                                         sanakirja,jammdb,
	MMAP Containers:                            memvec
	SERIALIZATION (zero-copy deserialization?): zerovec,rmp-serde
Design Patterns to build:
	Multi-Frontend app, follow RustBrake's nice Generics architecture
canvas:
	DomBalls speed comparison
Async:
	https://crates.io/crates/context-attribute might be useful to improve error reporting on async code
Backend Framework:
	Rocket (still in nightly, unfortunately...)
	try actix-web instead
