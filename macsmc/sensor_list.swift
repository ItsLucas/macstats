internal let SensorsList: [Sensor] = [
    // Temperature
    Sensor(key: "TA%P", name: "Ambient %", group: .sensor, type: .temperature, platforms: Platform.all),
    Sensor(key: "Th%H", name: "Heatpipe %", group: .sensor, type: .temperature, platforms: [.intel]),
    Sensor(key: "TZ%C", name: "Thermal zone %", group: .sensor, type: .temperature, platforms: Platform.all),
    
    Sensor(key: "TC0D", name: "CPU diode", group: .CPU, type: .temperature, platforms: Platform.all),
    Sensor(key: "TC0E", name: "CPU diode virtual", group: .CPU, type: .temperature, platforms: Platform.all),
    Sensor(key: "TC0F", name: "CPU diode filtered", group: .CPU, type: .temperature, platforms: Platform.all),
    Sensor(key: "TC0H", name: "CPU heatsink", group: .CPU, type: .temperature, platforms: Platform.all),
    Sensor(key: "TC0P", name: "CPU proximity", group: .CPU, type: .temperature, platforms: Platform.all),
    Sensor(key: "TCAD", name: "CPU package", group: .CPU, type: .temperature, platforms: Platform.all),
    
    Sensor(key: "TC%c", name: "CPU core %", group: .CPU, type: .temperature, platforms: Platform.all, average: true),
    Sensor(key: "TC%C", name: "CPU Core %", group: .CPU, type: .temperature, platforms: Platform.all, average: true),
    
    Sensor(key: "TCGC", name: "GPU Intel Graphics", group: .GPU, type: .temperature, platforms: Platform.all),
    Sensor(key: "TG0D", name: "GPU diode", group: .GPU, type: .temperature, platforms: Platform.all),
    Sensor(key: "TGDD", name: "GPU AMD Radeon", group: .GPU, type: .temperature, platforms: Platform.all),
    Sensor(key: "TG0H", name: "GPU heatsink", group: .GPU, type: .temperature, platforms: Platform.all),
    Sensor(key: "TG0P", name: "GPU proximity", group: .GPU, type: .temperature, platforms: Platform.all),
    
    Sensor(key: "Tm0P", name: "Mainboard", group: .system, type: .temperature, platforms: Platform.all),
    Sensor(key: "Tp0P", name: "Powerboard", group: .system, type: .temperature, platforms: [.intel]),
    Sensor(key: "TB1T", name: "Battery", group: .system, type: .temperature, platforms: [.intel]),
    Sensor(key: "TW0P", name: "Airport", group: .system, type: .temperature, platforms: Platform.all),
    Sensor(key: "TL0P", name: "Display", group: .system, type: .temperature, platforms: Platform.all),
    Sensor(key: "TI%P", name: "Thunderbolt %", group: .system, type: .temperature, platforms: Platform.all),
    Sensor(key: "TH%A", name: "Disk % (A)", group: .system, type: .temperature, platforms: Platform.all),
    Sensor(key: "TH%B", name: "Disk % (B)", group: .system, type: .temperature, platforms: Platform.all),
    Sensor(key: "TH%C", name: "Disk % (C)", group: .system, type: .temperature, platforms: Platform.all),
    
    Sensor(key: "TTLD", name: "Thunderbolt left", group: .system, type: .temperature, platforms: Platform.all),
    Sensor(key: "TTRD", name: "Thunderbolt right", group: .system, type: .temperature, platforms: Platform.all),
    
    Sensor(key: "TN0D", name: "Northbridge diode", group: .system, type: .temperature, platforms: Platform.all),
    Sensor(key: "TN0H", name: "Northbridge heatsink", group: .system, type: .temperature, platforms: Platform.all),
    Sensor(key: "TN0P", name: "Northbridge proximity", group: .system, type: .temperature, platforms: Platform.all),
    
    // Apple Silicon
    Sensor(key: "Tp09", name: "CPU efficiency core 1", group: .CPU, type: .temperature, platforms: Platform.m1Gen, average: true),
    Sensor(key: "Tp0T", name: "CPU efficiency core 2", group: .CPU, type: .temperature, platforms: Platform.m1Gen, average: true),
    Sensor(key: "Tp01", name: "CPU performance core 1", group: .CPU, type: .temperature, platforms: Platform.m1Gen, average: true),
    Sensor(key: "Tp05", name: "CPU performance core 2", group: .CPU, type: .temperature, platforms: Platform.m1Gen, average: true),
    Sensor(key: "Tp0D", name: "CPU performance core 3", group: .CPU, type: .temperature, platforms: Platform.m1Gen, average: true),
    Sensor(key: "Tp0H", name: "CPU performance core 4", group: .CPU, type: .temperature, platforms: Platform.m1Gen, average: true),
    Sensor(key: "Tp0L", name: "CPU performance core 5", group: .CPU, type: .temperature, platforms: Platform.m1Gen, average: true),
    Sensor(key: "Tp0P", name: "CPU performance core 6", group: .CPU, type: .temperature, platforms: Platform.m1Gen, average: true),
    Sensor(key: "Tp0X", name: "CPU performance core 7", group: .CPU, type: .temperature, platforms: Platform.m1Gen, average: true),
    Sensor(key: "Tp0b", name: "CPU performance core 8", group: .CPU, type: .temperature, platforms: Platform.m1Gen, average: true),
    
    Sensor(key: "Tg05", name: "GPU 1", group: .GPU, type: .temperature, platforms: Platform.m1Gen, average: true),
    Sensor(key: "Tg0D", name: "GPU 2", group: .GPU, type: .temperature, platforms: Platform.m1Gen, average: true),
    Sensor(key: "Tg0L", name: "GPU 3", group: .GPU, type: .temperature, platforms: Platform.m1Gen, average: true),
    Sensor(key: "Tg0T", name: "GPU 4", group: .GPU, type: .temperature, platforms: Platform.m1Gen, average: true),
    
    Sensor(key: "Tm02", name: "Memory 1", group: .sensor, type: .temperature, platforms: Platform.m1Gen),
    Sensor(key: "Tm06", name: "Memory 2", group: .sensor, type: .temperature, platforms: Platform.m1Gen),
    Sensor(key: "Tm08", name: "Memory 3", group: .sensor, type: .temperature, platforms: Platform.m1Gen),
    Sensor(key: "Tm09", name: "Memory 4", group: .sensor, type: .temperature, platforms: Platform.m1Gen),
    
    // M2
    Sensor(key: "Tp1h", name: "CPU efficiency core 1", group: .CPU, type: .temperature, platforms: Platform.m2Gen, average: true),
    Sensor(key: "Tp1t", name: "CPU efficiency core 2", group: .CPU, type: .temperature, platforms: Platform.m2Gen, average: true),
    Sensor(key: "Tp1p", name: "CPU efficiency core 3", group: .CPU, type: .temperature, platforms: Platform.m2Gen, average: true),
    Sensor(key: "Tp1l", name: "CPU efficiency core 4", group: .CPU, type: .temperature, platforms: Platform.m2Gen, average: true),
    
    Sensor(key: "Tp01", name: "CPU performance core 1", group: .CPU, type: .temperature, platforms: Platform.m2Gen, average: true),
    Sensor(key: "Tp05", name: "CPU performance core 2", group: .CPU, type: .temperature, platforms: Platform.m2Gen, average: true),
    Sensor(key: "Tp09", name: "CPU performance core 3", group: .CPU, type: .temperature, platforms: Platform.m2Gen, average: true),
    Sensor(key: "Tp0D", name: "CPU performance core 4", group: .CPU, type: .temperature, platforms: Platform.m2Gen, average: true),
    Sensor(key: "Tp0X", name: "CPU performance core 5", group: .CPU, type: .temperature, platforms: Platform.m2Gen, average: true),
    Sensor(key: "Tp0b", name: "CPU performance core 6", group: .CPU, type: .temperature, platforms: Platform.m2Gen, average: true),
    Sensor(key: "Tp0f", name: "CPU performance core 7", group: .CPU, type: .temperature, platforms: Platform.m2Gen, average: true),
    Sensor(key: "Tp0j", name: "CPU performance core 8", group: .CPU, type: .temperature, platforms: Platform.m2Gen, average: true),
    
    Sensor(key: "Tg0f", name: "GPU 1", group: .GPU, type: .temperature, platforms: Platform.m2Gen, average: true),
    Sensor(key: "Tg0j", name: "GPU 2", group: .GPU, type: .temperature, platforms: Platform.m2Gen, average: true),
    
    // M3
    Sensor(key: "Te05", name: "CPU efficiency core 1", group: .CPU, type: .temperature, platforms: Platform.m3Gen, average: true),
    Sensor(key: "Te0L", name: "CPU efficiency core 2", group: .CPU, type: .temperature, platforms: Platform.m3Gen, average: true),
    Sensor(key: "Te0P", name: "CPU efficiency core 3", group: .CPU, type: .temperature, platforms: Platform.m3Gen, average: true),
    Sensor(key: "Te0S", name: "CPU efficiency core 4", group: .CPU, type: .temperature, platforms: Platform.m3Gen, average: true),
    
    Sensor(key: "Tf04", name: "CPU performance core 1", group: .CPU, type: .temperature, platforms: Platform.m3Gen, average: true),
    Sensor(key: "Tf09", name: "CPU performance core 2", group: .CPU, type: .temperature, platforms: Platform.m3Gen, average: true),
    Sensor(key: "Tf0A", name: "CPU performance core 3", group: .CPU, type: .temperature, platforms: Platform.m3Gen, average: true),
    Sensor(key: "Tf0B", name: "CPU performance core 4", group: .CPU, type: .temperature, platforms: Platform.m3Gen, average: true),
    Sensor(key: "Tf0D", name: "CPU performance core 5", group: .CPU, type: .temperature, platforms: Platform.m3Gen, average: true),
    Sensor(key: "Tf0E", name: "CPU performance core 6", group: .CPU, type: .temperature, platforms: Platform.m3Gen, average: true),
    Sensor(key: "Tf44", name: "CPU performance core 7", group: .CPU, type: .temperature, platforms: Platform.m3Gen, average: true),
    Sensor(key: "Tf49", name: "CPU performance core 8", group: .CPU, type: .temperature, platforms: Platform.m3Gen, average: true),
    Sensor(key: "Tf4A", name: "CPU performance core 9", group: .CPU, type: .temperature, platforms: Platform.m3Gen, average: true),
    Sensor(key: "Tf4B", name: "CPU performance core 10", group: .CPU, type: .temperature, platforms: Platform.m3Gen, average: true),
    Sensor(key: "Tf4D", name: "CPU performance core 11", group: .CPU, type: .temperature, platforms: Platform.m3Gen, average: true),
    Sensor(key: "Tf4E", name: "CPU performance core 12", group: .CPU, type: .temperature, platforms: Platform.m3Gen, average: true),
    
    Sensor(key: "Tf14", name: "GPU 1", group: .GPU, type: .temperature, platforms: Platform.m3Gen, average: true),
    Sensor(key: "Tf18", name: "GPU 2", group: .GPU, type: .temperature, platforms: Platform.m3Gen, average: true),
    Sensor(key: "Tf19", name: "GPU 3", group: .GPU, type: .temperature, platforms: Platform.m3Gen, average: true),
    Sensor(key: "Tf1A", name: "GPU 4", group: .GPU, type: .temperature, platforms: Platform.m3Gen, average: true),
    Sensor(key: "Tf24", name: "GPU 5", group: .GPU, type: .temperature, platforms: Platform.m3Gen, average: true),
    Sensor(key: "Tf28", name: "GPU 6", group: .GPU, type: .temperature, platforms: Platform.m3Gen, average: true),
    Sensor(key: "Tf29", name: "GPU 7", group: .GPU, type: .temperature, platforms: Platform.m3Gen, average: true),
    Sensor(key: "Tf2A", name: "GPU 8", group: .GPU, type: .temperature, platforms: Platform.m3Gen, average: true),
    
    // M4
    Sensor(key: "Te05", name: "CPU efficiency core 1", group: .CPU, type: .temperature, platforms: Platform.m4Gen, average: true),
    Sensor(key: "Te0S", name: "CPU efficiency core 2", group: .CPU, type: .temperature, platforms: Platform.m4Gen, average: true),
    Sensor(key: "Te09", name: "CPU efficiency core 3", group: .CPU, type: .temperature, platforms: Platform.m4Gen, average: true),
    Sensor(key: "Te0H", name: "CPU efficiency core 4", group: .CPU, type: .temperature, platforms: Platform.m4Gen, average: true),
    
    Sensor(key: "Tp01", name: "CPU performance core 1", group: .CPU, type: .temperature, platforms: Platform.m4Gen, average: true),
    Sensor(key: "Tp05", name: "CPU performance core 2", group: .CPU, type: .temperature, platforms: Platform.m4Gen, average: true),
    Sensor(key: "Tp09", name: "CPU performance core 3", group: .CPU, type: .temperature, platforms: Platform.m4Gen, average: true),
    Sensor(key: "Tp0D", name: "CPU performance core 4", group: .CPU, type: .temperature, platforms: Platform.m4Gen, average: true),
    Sensor(key: "Tp0V", name: "CPU performance core 5", group: .CPU, type: .temperature, platforms: Platform.m4Gen, average: true),
    Sensor(key: "Tp0Y", name: "CPU performance core 6", group: .CPU, type: .temperature, platforms: Platform.m4Gen, average: true),
    Sensor(key: "Tp0b", name: "CPU performance core 7", group: .CPU, type: .temperature, platforms: Platform.m4Gen, average: true),
    Sensor(key: "Tp0e", name: "CPU performance core 8", group: .CPU, type: .temperature, platforms: Platform.m4Gen, average: true),
    
    Sensor(key: "Tg0G", name: "GPU 1", group: .GPU, type: .temperature, platforms: [.m4], average: true),
    Sensor(key: "Tg0H", name: "GPU 2", group: .GPU, type: .temperature, platforms: [.m4], average: true),
    Sensor(key: "Tg1U", name: "GPU 1", group: .GPU, type: .temperature, platforms: [.m4Pro, .m4Max, .m4Ultra], average: true),
    Sensor(key: "Tg1k", name: "GPU 2", group: .GPU, type: .temperature, platforms: [.m4Pro, .m4Max, .m4Ultra], average: true),
    
    Sensor(key: "Tg0K", name: "GPU 3", group: .GPU, type: .temperature, platforms: Platform.m4Gen, average: true),
    Sensor(key: "Tg0L", name: "GPU 4", group: .GPU, type: .temperature, platforms: Platform.m4Gen, average: true),
    Sensor(key: "Tg0d", name: "GPU 5", group: .GPU, type: .temperature, platforms: Platform.m4Gen, average: true),
    Sensor(key: "Tg0e", name: "GPU 6", group: .GPU, type: .temperature, platforms: Platform.m4Gen, average: true),
    Sensor(key: "Tg0j", name: "GPU 7", group: .GPU, type: .temperature, platforms: Platform.m4Gen, average: true),
    Sensor(key: "Tg0k", name: "GPU 8", group: .GPU, type: .temperature, platforms: Platform.m4Gen, average: true),
    
    Sensor(key: "Tm0p", name: "Memory Proximity 1", group: .sensor, type: .temperature, platforms: Platform.m4Gen),
    Sensor(key: "Tm1p", name: "Memory Proximity 2", group: .sensor, type: .temperature, platforms: Platform.m4Gen),
    Sensor(key: "Tm2p", name: "Memory Proximity 3", group: .sensor, type: .temperature, platforms: Platform.m4Gen),
    
    // Apple Silicon
    
    Sensor(key: "TaLP", name: "Airflow left", group: .sensor, type: .temperature, platforms: Platform.apple),
    Sensor(key: "TaRF", name: "Airflow right", group: .sensor, type: .temperature, platforms: Platform.apple),
    
    Sensor(key: "TH0x", name: "NAND", group: .system, type: .temperature, platforms: Platform.apple),
    Sensor(key: "TB1T", name: "Battery 1", group: .system, type: .temperature, platforms: Platform.apple),
    Sensor(key: "TB2T", name: "Battery 2", group: .system, type: .temperature, platforms: Platform.apple),
    Sensor(key: "TW0P", name: "Airport", group: .system, type: .temperature, platforms: Platform.apple),
    
    // Voltage
    Sensor(key: "VCAC", name: "CPU IA", group: .CPU, type: .voltage, platforms: Platform.all),
    Sensor(key: "VCSC", name: "CPU System Agent", group: .CPU, type: .voltage, platforms: Platform.all),
    Sensor(key: "VC%C", name: "CPU Core %", group: .CPU, type: .voltage, platforms: Platform.all),
    
    Sensor(key: "VCTC", name: "GPU Intel Graphics", group: .GPU, type: .voltage, platforms: Platform.all),
    Sensor(key: "VG0C", name: "GPU", group: .GPU, type: .voltage, platforms: Platform.all),
    
    Sensor(key: "VM0R", name: "Memory", group: .system, type: .voltage, platforms: Platform.all),
    Sensor(key: "Vb0R", name: "CMOS", group: .system, type: .voltage, platforms: Platform.all),
    
    Sensor(key: "VD0R", name: "DC In", group: .sensor, type: .voltage, platforms: Platform.all),
    Sensor(key: "VP0R", name: "12V rail", group: .sensor, type: .voltage, platforms: Platform.all),
    Sensor(key: "Vp0C", name: "12V vcc", group: .sensor, type: .voltage, platforms: Platform.all),
    Sensor(key: "VV2S", name: "3V", group: .sensor, type: .voltage, platforms: Platform.all),
    Sensor(key: "VR3R", name: "3.3V", group: .sensor, type: .voltage, platforms: Platform.all),
    Sensor(key: "VV1S", name: "5V", group: .sensor, type: .voltage, platforms: Platform.all),
    Sensor(key: "VV9S", name: "12V", group: .sensor, type: .voltage, platforms: Platform.all),
    Sensor(key: "VeES", name: "PCI 12V", group: .sensor, type: .voltage, platforms: Platform.all),
    
    // Current
    Sensor(key: "IC0R", name: "CPU High side", group: .sensor, type: .current, platforms: Platform.all),
    Sensor(key: "IG0R", name: "GPU High side", group: .sensor, type: .current, platforms: Platform.all),
    Sensor(key: "ID0R", name: "DC In", group: .sensor, type: .current, platforms: Platform.all),
    Sensor(key: "IBAC", name: "Battery", group: .sensor, type: .current, platforms: Platform.all),
    Sensor(key: "IDBR", name: "Brightness", group: .sensor, type: .current, platforms: Platform.all),
    Sensor(key: "IU1R", name: "Thunderbolt Left", group: .sensor, type: .current, platforms: Platform.all),
    Sensor(key: "IU2R", name: "Thunderbolt Right", group: .sensor, type: .current, platforms: Platform.all),
    
    // Power
    Sensor(key: "PC0C", name: "CPU Core", group: .CPU, type: .power, platforms: Platform.all),
    Sensor(key: "PCAM", name: "CPU Core (IMON)", group: .CPU, type: .power, platforms: Platform.all),
    Sensor(key: "PCPC", name: "CPU Package", group: .CPU, type: .power, platforms: Platform.all),
    Sensor(key: "PCTR", name: "CPU Total", group: .CPU, type: .power, platforms: Platform.all),
    Sensor(key: "PCPT", name: "CPU Package total", group: .CPU, type: .power, platforms: Platform.all),
    Sensor(key: "PCPR", name: "CPU Package total (SMC)", group: .CPU, type: .power, platforms: Platform.all),
    Sensor(key: "PC0R", name: "CPU Computing high side", group: .CPU, type: .power, platforms: Platform.all),
    Sensor(key: "PC0G", name: "CPU GFX", group: .CPU, type: .power, platforms: Platform.all),
    Sensor(key: "PCEC", name: "CPU VccEDRAM", group: .CPU, type: .power, platforms: Platform.all),
    
    Sensor(key: "PCPG", name: "GPU Intel Graphics", group: .GPU, type: .power, platforms: Platform.all),
    Sensor(key: "PG0C", name: "GPU", group: .GPU, type: .power, platforms: Platform.all),
    Sensor(key: "PG0R", name: "GPU 1", group: .GPU, type: .power, platforms: Platform.all),
    Sensor(key: "PG1R", name: "GPU 2", group: .GPU, type: .power, platforms: Platform.all),
    Sensor(key: "PCGC", name: "Intel GPU", group: .GPU, type: .power, platforms: Platform.all),
    Sensor(key: "PCGM", name: "Intel GPU (IMON)", group: .GPU, type: .power, platforms: Platform.all),
    
    Sensor(key: "PC3C", name: "RAM", group: .sensor, type: .power, platforms: Platform.all),
    Sensor(key: "PPBR", name: "Battery", group: .sensor, type: .power, platforms: Platform.all),
    Sensor(key: "PDTR", name: "DC In", group: .sensor, type: .power, platforms: Platform.all),
    Sensor(key: "PMTR", name: "Memory Total", group: .sensor, type: .power, platforms: Platform.all),
    Sensor(key: "PSTR", name: "System Total", group: .sensor, type: .power, platforms: Platform.all),
    
    Sensor(key: "PU1R", name: "Thunderbolt Left", group: .sensor, type: .power, platforms: Platform.all),
    Sensor(key: "PU2R", name: "Thunderbolt Right", group: .sensor, type: .power, platforms: Platform.all),
    
    Sensor(key: "PDBR", name: "Power Delivery Brightness", group: .sensor, type: .power, platforms: [.m1, .m1Pro, .m1Max, .m1Ultra, .m4, .m4Pro, .m4Max, .m4Ultra])
]