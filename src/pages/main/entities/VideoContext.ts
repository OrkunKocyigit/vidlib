import { type VideoFile } from '../../../entities/VideoFile';
import { createContext } from 'react';

export interface IVideoContext {
  video?: VideoFile;
  setVideo?: (video: VideoFile) => void;
}

export const VideoContext = createContext<IVideoContext>({});
