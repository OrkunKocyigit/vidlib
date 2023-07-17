import { FileInfo, type IFileInfo } from './FileInfo';

export interface IVideoFile extends IFileInfo {
  id: string;
}

export class VideoFile extends FileInfo implements IVideoFile {
  id: string;

  constructor(depth: number, name: string, path: string, id: string) {
    super(depth, name, path);
    this.id = id;
  }
}
