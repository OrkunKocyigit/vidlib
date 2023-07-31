import { FileInfo, type IFileInfo } from './FileInfo';
import { type VideoEntry } from './VideoEntry';

export interface IVideoFile extends IFileInfo {
  id: string;
  video?: VideoEntry;
}

export class VideoFile extends FileInfo implements IVideoFile {
  id: string;
  video?: VideoEntry;

  constructor(depth: number, name: string, path: string, id: string, video?: VideoEntry) {
    super(depth, name, path);
    this.id = id;
    this.video = video;
  }

  get displayName(): string {
    return this.name;
  }
}
