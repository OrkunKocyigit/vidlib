import { FileInfo, type IFileInfo } from './FileInfo';
import { type VideoEntry } from './VideoEntry';

export interface IVideoFile extends IFileInfo {
  id: string;
  video?: VideoEntry;
  watched: boolean;
}

export class VideoFile extends FileInfo implements IVideoFile {
  id: string;
  video?: VideoEntry;
  watched: boolean;

  constructor(
    depth: number,
    name: string,
    path: string,
    id: string,
    watched: boolean,
    video?: VideoEntry
  ) {
    super(depth, name, path);
    this.id = id;
    this.video = video;
    this.watched = watched;
  }

  get displayName(): string {
    return this.name;
  }
}
