import react from "react"
import {Box, Group, rem, TextInput, useMantineColorScheme} from "@mantine/core";
import {Search} from "tabler-icons-react";
export function SearchBar() {
    const { colorScheme, toggleColorScheme } = useMantineColorScheme();

    return (
        <Box sx={(theme) => ({
            paddingTop: theme.spacing.xs,
            paddingLeft: theme.spacing.xs,
            paddingRight: theme.spacing.xs,
            paddingBottom: theme.spacing.md,
            borderBottom: `${rem(1)} solid ${
                theme.colorScheme === 'dark' ? theme.colors.dark[4] : theme.colors.gray[2]
            }`
        })}>
            <Group position="apart">
                <TextInput placeholder="Filename" icon={<Search size="1rem"></Search>}></TextInput>
            </Group>
        </Box>
    )
}