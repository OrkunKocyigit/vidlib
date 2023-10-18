import { type FolderInfo } from '../../../entities/FolderInfo';
import FolderInfoView from './FolderInfoView';
import React from 'react';
import { useProps } from '@mantine/core';

export interface FileTreeViewProps extends React.ComponentPropsWithoutRef<'div'> {
  folders: FolderInfo[];
  showDelete?: boolean;
}

const defaultProps: Partial<FileTreeViewProps> = {
  showDelete: false
};

function FileTreeView(props: FileTreeViewProps): JSX.Element {
  const { className, folders, showDelete } = useProps('FileTreeView', defaultProps, props);
  return (
    <div className={className}>
      {folders.map((folder) => (
        <FolderInfoView
          folder={folder}
          key={`${folder.path}_${folder.depth}`}
          showDelete={showDelete}></FolderInfoView>
      ))}
    </div>
  );
}

export default FileTreeView;
