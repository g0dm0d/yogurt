import { useState } from 'react';
import { Navbar, Tooltip, UnstyledButton, createStyles, Stack } from '@mantine/core';
import {
    IconHome2,
    IconSettings,
    TablerIconsProps,
} from '@tabler/icons-react';
import { PlayerHead } from './ui/playerHead';
import { Link } from 'react-router-dom';

const useStyles = createStyles((theme) => ({
    link: {
        width: 50,
        height: 50,
        borderRadius: theme.radius.md,
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'center',
        color: theme.colorScheme === 'dark' ? theme.colors.dark[0] : theme.colors.gray[7],
        transition: '300ms',
        '&:hover': {
            backgroundColor: theme.colorScheme === 'dark' ? theme.colors.dark[0] : theme.colors.gray[0],
        },
    },

    active: {
        '&, &:hover': {
            backgroundColor: theme.fn.variant({ variant: 'light', color: theme.primaryColor }).background,
            color: theme.fn.variant({ variant: 'light', color: theme.primaryColor }).color,
        },
    },
}));


interface DashboardButtonProps {
    Icon: TablerIconsProps;
    label: string;
    active?: boolean;
    onClick?(): void;
}

function DashboardButton({ Icon: Icon, label, active, onClick }: DashboardButtonProps) {
    const { classes, cx } = useStyles();
    return (
        <Tooltip label={label} position="right" transitionDuration={0}>
            <UnstyledButton onClick={onClick} className={cx(classes.link, { [classes.active]: active })}>
                <Icon stroke={1.5} />
            </UnstyledButton>
        </Tooltip>
    );
}

export function Dashboard() {
    const [active, setActive] = useState(1);
    const { classes } = useStyles();
    return (
        <Navbar height="100vh" width={{ base: 80 }} p="md" sx={{border: 'none'}}>
            <Navbar.Section>
                <Link to='/account'>
                    <PlayerHead />
                </Link>
            </Navbar.Section>
            <Navbar.Section grow mt={32}>
                <Stack justify="center" spacing={8}>
                    <Link to=''>
                        <DashboardButton Icon={IconHome2} label='Home' active={active === 1} onClick={() => setActive(1)} />
                    </Link>
                    <Link to='settings'>
                        <DashboardButton Icon={IconSettings} label='Settings' active={active === 2} onClick={() => setActive(2)} />
                    </Link>
                </Stack>
            </Navbar.Section>
            {/* <Navbar.Section>
                <Stack justify="center" spacing={0}>
                    <DashboardButton Icon={IconSwitchHorizontal} label="Change account" />
                    <DashboardButton Icon={IconLogout} label="Logout" />
                </Stack>
            </Navbar.Section> */}
        </Navbar >
    );
}