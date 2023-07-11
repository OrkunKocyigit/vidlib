import {FolderInfo} from "../../../entities/FolderInfo";

type Props = {
    folders: FolderInfo[]
}

export default function FileTreeView(props: Props) {
    return (
        <pre>
            {JSON.stringify(props.folders, null, 4)}
        </pre>
    )
}