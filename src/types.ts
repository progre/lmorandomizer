export interface InitialParameters {
  seed: string;
  installDirectory: string;
  easyMode: boolean;
  tabletSave: boolean;
  grailStart: boolean;
  scannerStart: boolean;
  gameMasterStart: boolean;
  readerStart: boolean;
}

export interface Settings {
  seed?: string;
  installDirectory?: string;
  easyMode?: boolean;
  tabletSave?: boolean;
  grailStart?: boolean;
  scannerStart?: boolean;
  gameMasterStart?: boolean;
  readerStart?: boolean;
}
