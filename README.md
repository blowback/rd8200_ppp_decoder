# RD8200 PPP Decoder

Decode the PPP messages that the RadioDetection RD8200 (or any RD8x00 series locator) emits when
the SURVEY button is pressed.

The document on which this is based is dated 2019 and no more recent version seems to be available,
so this may not be accurate for RD8200 data.

## Known issues

### GPS time
There have been two GPS date rollovers; the RD8200 compensates for the first, but not the second. This
means the year value will be out by up to 19.6 years. The app compensates for this.

### Additional data
With an RD8200, I cannot consistently decode sane Additional Data readings. The spec is a little vague,
but it seems likely that Additional Data is little-endian (as all other RD data), and that bit 31 is the
Most Significant Bit. (The `--big-endian` and `--msb0` options exist to allow you to experiment with the
other possible permutations.)

However even if these assumptions are correct I can't get stable sane readings: for
example RD8200 frequently identifies as Protocol ID "RDMRX", but occasionally "RD8100". We also see the
"Sonde" bit set, when I'm pretty sure that wasn't selected on the device.

My suspicion is that the spec for "Additional Data" was updated for RD8200, and we don't have enough
information to decode it properly. We'd need some very controlled tests (varying the fields in Additional
Data in very precise and controlled ways) to get to the bottom of what's going on.

## Usage

```
Decode and display PPP format packets from RadioDetection's RD8x000 series locators

Usage: rd8200_ppp_decoder [OPTIONS] [PATHS]...

Arguments:
  [PATHS]...  Paths of binary RD8x00 PPP files to process

Options:
  -d, --debug...    Enable debugging, specify 2 or 3 times to get more output
      --big-endian  Treat "additional data" as a big-endian u32
      --msb0        Treat "additional data" as having most-significant bit at bit position 0
      --no-rd       Suppress the output of the "RD" data section
      --no-loc      Suppress the output of the "New Locator Data" section
      --no-mrx      Suppress the output of the "MRX" section
      --no-rtc      Suppress the output of the "RTC" section
      --no-gps      Suppress the output of the "GPS" section
  -h, --help        Print help
  -V, --version     Print version
  ```

  ## Example output

```

File: data/nick_helpful_video.bin
frame: 0: [
	RD: {mode: Active, freq: 8192 Hz, depth: 0m, FF: 0 dBm, cur: 0 A, CD: 0°, SS: 0.0002794235, gain: 68 dB xtra:[RD8100 62° lr=R ant=PeakPlus acc=None Line bat=Medium vol=Off Noverload ]}
	LOC: {Internal GPS}
	MRX: {typ:Unused depth:0m sig:0 gain:0dB}
	RTC: {2023-11-2 18:49:57}
	GPS: {2023-11-2 18:49:58 hdop:99.99 alt:0??? height:0??? fix:None #sats:0 lat:0° lon:0°}
]
File: data/bad.ppp.bin
frame: 0: [
	RD: {mode: Active, freq: 8192 Hz, depth: 0m, FF: 0 dBm, cur: 0 A, CD: 0°, SS: 0.00000010003047, gain: 140 dB xtra:[RDMRX 14° lr=PeakMode ant=PeakPlus acc=None Sonde bat=High vol=Maximum Noverload ]}
	LOC: {Internal GPS}
	MRX: {typ:Unused depth:0m sig:0 gain:0dB}
	RTC: {2023-10-20 14:53:37}
	GPS: {2023-10-20 14:53:36 hdop:0.79 alt:16m height:-27.5m fix:GPS #sats:10 lat:26.214553333333335° lon:-81.78488333333334°}
]
frame: 1: [
	RD: {mode: Active, freq: 8192 Hz, depth: 0m, FF: 0 dBm, cur: 0 A, CD: 0°, SS: 0.00000010000017, gain: 140 dB xtra:[RDMRX 10° lr=PeakMode ant=PeakPlus acc=None Sonde bat=High vol=Maximum Noverload ]}
	LOC: {Internal GPS}
	MRX: {typ:Unused depth:0m sig:0 gain:0dB}
	RTC: {2023-10-20 14:54:09}
	GPS: {2023-10-20 14:54:09 hdop:0.85 alt:16.1m height:-27.5m fix:GPS #sats:9 lat:26.214555° lon:-81.78487166666666°}
]
frame: 2: [
	RD: {mode: Active, freq: 570 Hz, depth: 0m, FF: 0 dBm, cur: 0 A, CD: 0°, SS: 0.00024815803, gain: 78 dB xtra:[RDMRX 89° lr=PeakMode ant=Peak acc=None Line bat=High vol=Maximum Noverload ]}
	LOC: {Internal GPS}
	MRX: {typ:Unused depth:0m sig:0 gain:0dB}
	RTC: {2023-10-20 14:54:55}
	GPS: {2023-10-20 14:54:55 hdop:0.79 alt:15.3m height:-27.5m fix:GPS #sats:10 lat:26.214553333333335° lon:-81.78482666666666°}
]
frame: 3: [
	RD: {mode: Active, freq: 8192 Hz, depth: 0m, FF: 0 dBm, cur: 0 A, CD: 0°, SS: 0.17588794, gain: 36 dB xtra:[RDMRX 60° lr=R ant=VerticalNone acc=None Sonde bat=High vol=Maximum Noverload ]}
	LOC: {Internal GPS}
	MRX: {typ:Unused depth:0m sig:0 gain:0dB}
	RTC: {2023-10-20 14:55:27}
	GPS: {2023-10-20 14:55:27 hdop:0.85 alt:15.2m height:-27.5m fix:GPS #sats:9 lat:26.21456° lon:-81.78481333333333°}
]
frame: 4: [
	RD: {mode: Active, freq: 9820 Hz, depth: 0m, FF: 0 dBm, cur: 0 A, CD: 0°, SS: 0.0002298556, gain: 59 dB xtra:[RDMRX 22° lr=PeakMode ant=PeakPlus acc=None Line bat=High vol=Maximum Noverload ]}
	LOC: {Internal GPS}
	MRX: {typ:Unused depth:0m sig:0 gain:0dB}
	RTC: {2023-10-20 14:56:09}
	GPS: {2023-10-20 14:56:09 hdop:0.85 alt:15.1m height:-27.5m fix:GPS #sats:9 lat:26.21456166666667° lon:-81.784785°}
]
frame: 5: [
	RD: {mode: Active, freq: 9820 Hz, depth: 0m, FF: 0 dBm, cur: 0 A, CD: 0°, SS: 0.76030827, gain: 119 dB xtra:[RDMRX 42° lr=PeakMode ant=VerticalNone acc=None Line bat=High vol=Maximum Noverload ]}
	LOC: {Internal GPS}
	MRX: {typ:Unused depth:0m sig:0 gain:0dB}
	RTC: {2023-10-20 14:56:38}
	GPS: {2023-10-20 14:56:38 hdop:0.85 alt:14.3m height:-27.5m fix:GPS #sats:9 lat:26.214575° lon:-81.784755°}
]
frame: 6: [
	RD: {mode: Active, freq: 32768 Hz, depth: 0m, FF: 0 dBm, cur: 0 A, CD: 0°, SS: 0.6807083, gain: 140 dB xtra:[RDMRX 306° lr=LLLLL ant=PeakPlus acc=None Line bat=High vol=Maximum Noverload ]}
	LOC: {Internal GPS}
	MRX: {typ:Unused depth:0m sig:0 gain:0dB}
	RTC: {2023-10-20 14:57:21}
	GPS: {2023-10-20 14:57:21 hdop:0.85 alt:13.8m height:-27.5m fix:GPS #sats:9 lat:26.214563333333334° lon:-81.78478666666666°}
]
frame: 7: [
	RD: {mode: CDInverted, freq: 1168 Hz, depth: 0m, FF: 0 dBm, cur: 0 A, CD: 91.82185°, SS: 0.00000010000017, gain: 140 dB xtra:[RDMRX 76° lr=PeakMode ant=Peak acc=None Line bat=High vol=Maximum Noverload ]}
	LOC: {Internal GPS}
	MRX: {typ:Unused depth:0m sig:0 gain:0dB}
	RTC: {2023-10-20 14:58:01}
	GPS: {2023-10-20 14:58:00 hdop:0.85 alt:13.7m height:-27.5m fix:GPS #sats:9 lat:26.214613333333332° lon:-81.78475833333333°}
]
frame: 8: [
	RD: {mode: Active, freq: 8440 Hz, depth: 0m, FF: 0 dBm, cur: 0 A, CD: 0°, SS: 0.9987018, gain: 140 dB xtra:[RDMRX 81° lr=PeakMode ant=Broad acc=None Line bat=High vol=Maximum Noverload ]}
	LOC: {Internal GPS}
	MRX: {typ:Unused depth:0m sig:0 gain:0dB}
	RTC: {2023-10-20 14:58:37}
	GPS: {2023-10-20 14:58:37 hdop:0.85 alt:13.7m height:-27.5m fix:GPS #sats:9 lat:26.214621666666666° lon:-81.78473166666667°}
]
frame: 9: [
	RD: {mode: Active, freq: 8440 Hz, depth: 0m, FF: 0 dBm, cur: 0 A, CD: 0°, SS: 0.1424212, gain: 60 dB xtra:[RDMRX 338° lr=L ant=Guidance acc=None Line bat=High vol=Maximum Noverload ]}
	LOC: {Internal GPS}
	MRX: {typ:Unused depth:0m sig:0 gain:0dB}
	RTC: {2023-10-20 14:59:03}
	GPS: {2023-10-20 14:59:03 hdop:0.85 alt:13.2m height:-27.5m fix:GPS #sats:9 lat:26.21457° lon:-81.78470166666666°}
]
frame: 10: [
	RD: {mode: Active, freq: 8440 Hz, depth: 0m, FF: 0 dBm, cur: 0 A, CD: 0°, SS: 0.07925493, gain: 60 dB xtra:[RDMRX 326° lr=L ant=Guidance acc=None Line bat=High vol=Maximum Noverload ]}
	LOC: {Internal GPS}
	MRX: {typ:Unused depth:0m sig:0 gain:0dB}
	RTC: {2023-10-20 14:59:54}
	GPS: {2023-10-20 14:59:54 hdop:0.85 alt:12.8m height:-27.5m fix:GPS #sats:9 lat:26.214506666666665° lon:-81.78479°}
]
frame: 11: [
	RD: {mode: Active, freq: 8440 Hz, depth: 0m, FF: 0 dBm, cur: 0 A, CD: 0°, SS: 0.86477727, gain: 140 dB xtra:[RDMRX 25° lr=PeakMode ant=PeakPlus acc=None Line bat=High vol=Maximum Noverload ]}
	LOC: {Internal GPS}
	MRX: {typ:Unused depth:0m sig:0 gain:0dB}
	RTC: {2023-10-20 15:00:32}
	GPS: {2023-10-20 15:00:32 hdop:0.85 alt:12.9m height:-27.5m fix:GPS #sats:9 lat:26.214585° lon:-81.784855°}
]
File: data/good_front_of_building.ppp.bin
frame: 0: [
	RD: {mode: Active, freq: 8192 Hz, depth: 3.442315m, FF: 0 dBm, cur: 0 A, CD: 0°, SS: 0.5088795, gain: 69 dB xtra:[RDMRX 2° lr=Centre ant=PeakPlus acc=None Sonde bat=High vol=Maximum Noverload ]}
	LOC: {Internal GPS}
	MRX: {typ:Unused depth:0m sig:0 gain:0dB}
	RTC: {2019-8-17 00:16:05}
	GPS: {2023-10-20 14:21:33 hdop:0 alt:0m height:0m fix:None #sats:0 lat:0° lon:0°}
]
frame: 1: [
	RD: {mode: Active, freq: 8192 Hz, depth: 3.4285092m, FF: 0 dBm, cur: 0 A, CD: 0°, SS: 0.56934565, gain: 69 dB xtra:[RDMRX 348° lr=Centre ant=PeakPlus acc=None Sonde bat=High vol=Maximum Noverload ]}
	LOC: {Internal GPS}
	MRX: {typ:Unused depth:0m sig:0 gain:0dB}
	RTC: {2019-8-17 00:16:36}
	GPS: {2023-10-20 14:22:04 hdop:0 alt:0m height:0m fix:None #sats:0 lat:0° lon:0°}
]
frame: 2: [
	RD: {mode: Active, freq: 8192 Hz, depth: 3.2824337m, FF: 0 dBm, cur: 0 A, CD: 0°, SS: 0.6056439, gain: 69 dB xtra:[RDMRX 358° lr=Centre ant=PeakPlus acc=None Sonde bat=High vol=Maximum Noverload ]}
	LOC: {Internal GPS}
	MRX: {typ:Unused depth:0m sig:0 gain:0dB}
	RTC: {2019-8-17 00:20:07}
	GPS: {2023-10-20 14:25:35 hdop:2.46 alt:177.6m height:-27.5m fix:GPS #sats:3 lat:26.213763333333333° lon:-81.78627833333333°}
]
frame: 3: [
	RD: {mode: Active, freq: 8192 Hz, depth: 3.3272157m, FF: 0 dBm, cur: 0 A, CD: 0°, SS: 0.6017315, gain: 69 dB xtra:[RDMRX 353° lr=Centre ant=PeakPlus acc=None Sonde bat=High vol=Maximum Noverload ]}
	LOC: {Internal GPS}
	MRX: {typ:Unused depth:0m sig:0 gain:0dB}
	RTC: {2023-10-20 14:27:34}
	GPS: {2023-10-20 14:27:34 hdop:1.21 alt:122.4m height:-27.5m fix:GPS #sats:5 lat:26.214368333333333° lon:-81.78488333333334°}
]
frame: 4: [
	RD: {mode: Active, freq: 8192 Hz, depth: 3.4975986m, FF: 0 dBm, cur: 0 A, CD: 0°, SS: 0.6145068, gain: 69 dB xtra:[RDMRX 350° lr=Centre ant=PeakPlus acc=None Sonde bat=High vol=Maximum Noverload ]}
	LOC: {Internal GPS}
	MRX: {typ:Unused depth:0m sig:0 gain:0dB}
	RTC: {2023-10-20 14:29:41}
	GPS: {2023-10-20 14:29:41 hdop:1.11 alt:120.1m height:-27.5m fix:GPS #sats:6 lat:26.214273333333335° lon:-81.7849°}
]
frame: 5: [
	RD: {mode: Active, freq: 8192 Hz, depth: 3.1433866m, FF: 0 dBm, cur: 0 A, CD: 0°, SS: 0.5011284, gain: 69 dB xtra:[RDMRX 355° lr=Centre ant=PeakPlus acc=None Sonde bat=High vol=Maximum Noverload ]}
	LOC: {Internal GPS}
	MRX: {typ:Unused depth:0m sig:0 gain:0dB}
	RTC: {2023-10-20 14:32:28}
	GPS: {2023-10-20 14:32:28 hdop:1.49 alt:106.1m height:-27.5m fix:GPS #sats:5 lat:26.214016666666666° lon:-81.785505°}
]
frame: 6: [
	RD: {mode: Active, freq: 8192 Hz, depth: 3.2280064m, FF: 0 dBm, cur: 0 A, CD: 0°, SS: 0.503981, gain: 69 dB xtra:[RDMRX 356° lr=Centre ant=PeakPlus acc=None Sonde bat=High vol=Maximum Noverload ]}
	LOC: {Internal GPS}
	MRX: {typ:Unused depth:0m sig:0 gain:0dB}
	RTC: {2023-10-20 14:33:17}
	GPS: {2023-10-20 14:33:17 hdop:1.11 alt:103.1m height:-27.5m fix:GPS #sats:6 lat:26.213996666666667° lon:-81.785485°}
]
frame: 7: [
	RD: {mode: Active, freq: 8192 Hz, depth: 2.7362208m, FF: 0 dBm, cur: 0 A, CD: 0°, SS: 0.45021725, gain: 69 dB xtra:[RDMRX 356° lr=Centre ant=PeakPlus acc=None Sonde bat=High vol=Maximum Noverload ]}
	LOC: {Internal GPS}
	MRX: {typ:Unused depth:0m sig:0 gain:0dB}
	RTC: {2023-10-20 14:34:16}
	GPS: {2023-10-20 14:34:16 hdop:1.49 alt:95.4m height:-27.5m fix:GPS #sats:5 lat:26.21407° lon:-81.78542°}
]
frame: 8: [
	RD: {mode: Active, freq: 8192 Hz, depth: 3.0886912m, FF: 0 dBm, cur: 0 A, CD: 0°, SS: 0.43347022, gain: 69 dB xtra:[RDMRX 357° lr=Centre ant=PeakPlus acc=None Sonde bat=High vol=Maximum Noverload ]}
	LOC: {Internal GPS}
	MRX: {typ:Unused depth:0m sig:0 gain:0dB}
	RTC: {2023-10-20 14:34:48}
	GPS: {2023-10-20 14:34:48 hdop:1.49 alt:94.8m height:-27.5m fix:GPS #sats:5 lat:26.21406° lon:-81.78546°}
]
File: data/good_in_a_line.ppp.bin
frame: 0: [
	RD: {mode: Active, freq: 8192 Hz, depth: 1.0914825m, FF: 0 dBm, cur: 0 A, CD: 0°, SS: 0.4398328, gain: 29 dB xtra:[RDMRX 357° lr=Centre ant=PeakPlus acc=None Sonde bat=High vol=Maximum Noverload ]}
	LOC: {Internal GPS}
	MRX: {typ:Unused depth:0m sig:0 gain:0dB}
	RTC: {2023-10-20 14:42:29}
	GPS: {2023-10-20 14:42:29 hdop:0.92 alt:32.1m height:-27.5m fix:GPS #sats:8 lat:26.214513333333333° lon:-81.78474666666666°}
]
frame: 1: [
	RD: {mode: Active, freq: 8192 Hz, depth: 1.0813512m, FF: 0 dBm, cur: 0 A, CD: 0°, SS: 0.44831297, gain: 29 dB xtra:[RDMRX 0° lr=Centre ant=PeakPlus acc=None Sonde bat=High vol=Maximum Noverload ]}
	LOC: {Internal GPS}
	MRX: {typ:Unused depth:0m sig:0 gain:0dB}
	RTC: {2023-10-20 14:42:57}
	GPS: {2023-10-20 14:42:57 hdop:0.85 alt:31.9m height:-27.5m fix:GPS #sats:9 lat:26.214503333333333° lon:-81.78474333333334°}
]
frame: 2: [
	RD: {mode: Active, freq: 8192 Hz, depth: 1.0368634m, FF: 0 dBm, cur: 0 A, CD: 0°, SS: 0.46481836, gain: 29 dB xtra:[RDMRX 0° lr=Centre ant=PeakPlus acc=None Sonde bat=High vol=Maximum Noverload ]}
	LOC: {Internal GPS}
	MRX: {typ:Unused depth:0m sig:0 gain:0dB}
	RTC: {2023-10-20 14:45:05}
	GPS: {2023-10-20 14:45:05 hdop:0.85 alt:29.2m height:-27.5m fix:GPS #sats:9 lat:26.214525° lon:-81.78471666666667°}
]
frame: 3: [
	RD: {mode: Active, freq: 8192 Hz, depth: 1.0422623m, FF: 0 dBm, cur: 0 A, CD: 0°, SS: 0.46159294, gain: 29 dB xtra:[RDMRX 2° lr=Centre ant=PeakPlus acc=None Sonde bat=High vol=Maximum Noverload ]}
	LOC: {Internal GPS}
	MRX: {typ:Unused depth:0m sig:0 gain:0dB}
	RTC: {2023-10-20 14:45:42}
	GPS: {2023-10-20 14:45:42 hdop:0.85 alt:26.5m height:-27.5m fix:GPS #sats:9 lat:26.214566666666666° lon:-81.78467666666667°}
]
frame: 4: [
	RD: {mode: Active, freq: 8192 Hz, depth: 0.9846948m, FF: 0 dBm, cur: 0 A, CD: 0°, SS: 0.5135343, gain: 29 dB xtra:[RDMRX 359° lr=Centre ant=PeakPlus acc=None Sonde bat=High vol=Maximum Noverload ]}
	LOC: {Internal GPS}
	MRX: {typ:Unused depth:0m sig:0 gain:0dB}
	RTC: {2023-10-20 14:47:02}
	GPS: {2023-10-20 14:47:02 hdop:0.85 alt:23m height:-27.5m fix:GPS #sats:9 lat:26.214618333333334° lon:-81.78463333333333°}
]
frame: 5: [
	RD: {mode: Active, freq: 8192 Hz, depth: 1.092199m, FF: 0 dBm, cur: 0 A, CD: 0°, SS: 0.42468244, gain: 29 dB xtra:[RDMRX 2° lr=Centre ant=PeakPlus acc=None Sonde bat=High vol=Maximum Noverload ]}
	LOC: {Internal GPS}
	MRX: {typ:Unused depth:0m sig:0 gain:0dB}
	RTC: {2023-10-20 14:47:39}
	GPS: {2023-10-20 14:47:39 hdop:0.85 alt:22.4m height:-27.5m fix:GPS #sats:9 lat:26.214611666666666° lon:-81.78462833333333°}
]
frame: 6: [
	RD: {mode: Active, freq: 8192 Hz, depth: 1.0370827m, FF: 0 dBm, cur: 0 A, CD: 0°, SS: 0.45148453, gain: 29 dB xtra:[RDMRX 0° lr=Centre ant=PeakPlus acc=None Sonde bat=High vol=Maximum Noverload ]}
	LOC: {Internal GPS}
	MRX: {typ:Unused depth:0m sig:0 gain:0dB}
	RTC: {2023-10-20 14:48:25}
	GPS: {2023-10-20 14:48:25 hdop:0.85 alt:20.9m height:-27.5m fix:GPS #sats:9 lat:26.214645° lon:-81.78458833333333°}
]
frame: 7: [
	RD: {mode: Active, freq: 8192 Hz, depth: 1.1010072m, FF: 0 dBm, cur: 0 A, CD: 0°, SS: 0.43384564, gain: 29 dB xtra:[RDMRX 359° lr=Centre ant=PeakPlus acc=None Sonde bat=High vol=Maximum Noverload ]}
	LOC: {Internal GPS}
	MRX: {typ:Unused depth:0m sig:0 gain:0dB}
	RTC: {2023-10-20 14:48:58}
	GPS: {2023-10-20 14:48:58 hdop:0.85 alt:19.7m height:-27.5m fix:GPS #sats:9 lat:26.21463° lon:-81.78456166666666°}
]
```


