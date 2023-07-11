import {FolderInfo} from "../../../entities/FolderInfo";
import FolderInfoView from "./FolderInfoView";
import React from "react"

type Props = {
    folders: FolderInfo[]
}

function FileTreeView(props: Props) {
    return (
        <div>
            {props.folders.map(folder => <FolderInfoView folder={folder} key={folder.path}></FolderInfoView>)}
        </div>
    )
}

export default FileTreeView