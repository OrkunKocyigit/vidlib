import { FileInfo, IFileInfo } from './FileInfo';

export interface IVideoFile extends IFileInfo {}

export class VideoFile extends FileInfo implements IVideoFile {}
