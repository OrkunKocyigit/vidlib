import { type FolderInfo } from '../../../entities/FolderInfo';
import FolderInfoView from './FolderInfoView';
import React from 'react';

interface FileTreeViewProps extends React.ComponentPropsWithoutRef<'div'> {
  folders: FolderInfo[];
}

function FileTreeView(props: FileTreeViewProps): JSX.Element {
  return (
    <div className={props.className}>
      {props.folders.map((folder) => (
        <FolderInfoView folder={folder} key={folder.path}></FolderInfoView>
      ))}
    </div>
  );
}

export default FileTreeView;
