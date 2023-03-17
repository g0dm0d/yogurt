import {
    ReactNode,
    useState,
    useEffect,
    createContext,
    useContext
} from 'react';
import {
    Navbar,
    Tooltip,
    UnstyledButton,
    createStyles,
    Stack,
    Title,
    Box
} from '@mantine/core';
import {
    IconCalendarStats,
    IconDeviceDesktopAnalytics,
    IconFingerprint,
    IconGauge,
    IconHome2,
    IconSettings,
    IconUser,
} from '@tabler/icons-react';

const useStyles = createStyles((theme) => ({
    wrapper: {
        display: 'flex',
    },

    aside: {
        width: 60,
        gap: 4,
        backgroundColor: theme.colorScheme === 'dark' ? theme.colors.dark[7] : theme.white,
        display: 'flex',
        flexDirection: 'column',
        alignItems: 'center',
        borderRight: `1 solid ${theme.colorScheme === 'dark' ? theme.colors.dark[7] : theme.colors.gray[3]}`,
    },

    main: {
        flex: 1,
        backgroundColor: theme.colorScheme === 'dark' ? theme.colors.dark[6] : theme.colors.gray[0],
    },

    Link: {
        width: '44px',
        height: '44px',
        borderRadius: theme.radius.md,
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'center',
        color: theme.colorScheme === 'dark' ? theme.colors.dark[0] : theme.colors.gray[7],
        transition: '300ms',
        '&:hover': {
            backgroundColor: theme.colorScheme === 'dark' ? theme.colors.dark[5] : theme.colors.gray[0],
        },
    },

    LinkActive: {
        '&, &:hover': {
            backgroundColor: theme.fn.variant({ variant: 'light', color: theme.primaryColor }).background,
            color: theme.fn.variant({ variant: 'light', color: theme.primaryColor }).color,
        },
    },

    title: {
        boxSizing: 'border-box',
        fontFamily: `Greycliff CF, ${theme.fontFamily}`,
        marginBottom: theme.spacing.xl,
        backgroundColor: theme.colorScheme === 'dark' ? theme.colors.dark[7] : theme.white,
        padding: theme.spacing.md,
        paddingTop: 18,
        height: 60,
        borderBottom: `1 solid ${theme.colorScheme === 'dark' ? theme.colors.dark[7] : theme.colors.gray[3]
            }`,
    },

    logo: {
        boxSizing: 'border-box',
        width: '100%',
        display: 'flex',
        justifyContent: 'center',
        height: '60px',
        paddingTop: theme.spacing.md,
        borderBottom: `1 solid ${theme.colorScheme === 'dark' ? theme.colors.dark[7] : theme.colors.gray[3]
            }`,
        marginBottom: theme.spacing.xl,
    },
}));

const LinksData = [
    { icon: IconHome2, label: 'Home' },
    { icon: IconGauge, label: 'Dashboard' },
    { icon: IconDeviceDesktopAnalytics, label: 'Analytics' },
    { icon: IconCalendarStats, label: 'Releases' },
    { icon: IconUser, label: 'Account' },
    { icon: IconFingerprint, label: 'Security' },
    { icon: IconSettings, label: 'Settings' },
];

export function SettingsDashboard() {
    const { classes, cx } = useStyles();
    const [active, setActive] = useState('Releases');
    const [activeLink, setActiveLink] = useState('Settings');

    const Links = LinksData.map((link) => (
        <Tooltip
            label={link.label}
            position="right"
            withArrow
            key={link.label}
        >
            <UnstyledButton
                onClick={() => setActive(link.label)}
                className={cx(classes.Link, { [classes.LinkActive]: link.label === active })}
            >
                <link.icon size="1.4rem" stroke={1.5} />
            </UnstyledButton>
        </Tooltip>
    ));

    return (
        <Navbar height={750} width={{ sm: 300 }}>
            <Navbar.Section grow className={classes.wrapper}>
                <Box className={classes.aside}>
                    <Box className={classes.logo}>
                        {/* <IconSettings type="mark" size={30} /> */}
                        Settings
                    </Box>
                    {Links}
                </Box>
            </Navbar.Section>
        </Navbar>
    );
}