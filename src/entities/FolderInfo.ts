import { type IVideoFile, VideoFile } from './VideoFile';
import { FileInfo, type IFileInfo } from './FileInfo';

export interface IFolderInfo extends IFileInfo {
  folders: IFolderInfo[];
  videos: IVideoFile[];
  empty: boolean;
}

export class FolderInfo extends FileInfo implements IFolderInfo {
  empty: boolean;
  folders: FolderInfo[];
  videos: VideoFile[];

  constructor(
    depth: number,
    name: string,
    path: string,
    empty: boolean,
    folders: IFolderInfo[],
    videos: IVideoFile[]
  ) {
    super(depth, name, path);
    this.empty = empty;
    this.folders = folders.map(
      ({ depth, name, path, empty, folders, videos }) =>
        new FolderInfo(depth, name, path, empty, folders, videos)
    );
    this.videos = videos.map(({ depth, name, path }) => new VideoFile(depth, name, path));
  }
}
