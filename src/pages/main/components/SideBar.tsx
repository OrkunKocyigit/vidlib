import React from "react";
import {FolderInfo} from "../../../entities/FolderInfo";
import {ActionIcon, Navbar} from "@mantine/core";
import {SearchBar} from "../../../components/SearchBar";
import {Plus} from "tabler-icons-react";

type Props = {
    folders: FolderInfo[]
}

type State = {

}
class SideBar extends React.Component<Props, State> {
    constructor(props: FolderInfo) {
        super(props);
    }

    render() {
        return (
            <Navbar width={{
                base: 300
            }}>
                <Navbar.Section>
                    <SearchBar>
                        <ActionIcon variant="default"><Plus size="1rem" /></ActionIcon>
                    </SearchBar>
                </Navbar.Section>
                <Navbar.Section grow>
                    {JSON.stringify(this.props.folders, null, 4)}
                </Navbar.Section>
            </Navbar>
        )
    }
}

export default SideBar