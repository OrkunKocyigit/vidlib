import { type FolderInfo } from '../../../entities/FolderInfo';
import FolderInfoView from './FolderInfoView';
import React from 'react';

interface Props {
  folders: FolderInfo[];
}

function FileTreeView(props: Props): JSX.Element {
  return (
    <div>
      {props.folders.map((folder) => (
        <FolderInfoView folder={folder} key={folder.path}></FolderInfoView>
      ))}
    </div>
  );
}

export default FileTreeView;
