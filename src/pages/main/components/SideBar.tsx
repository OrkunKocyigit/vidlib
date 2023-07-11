import React from "react";
import {FolderInfo} from "../../../entities/FolderInfo";
import {Navbar} from "@mantine/core";
import {SearchBar} from "../../../components/SearchBar";

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
            <Navbar>
                <Navbar.Section>
                    <SearchBar></SearchBar>
                </Navbar.Section>
                <Navbar.Section grow>
                    {JSON.stringify(this.props.folders, null, 4)}
                </Navbar.Section>
            </Navbar>
        )
    }
}

export default SideBar