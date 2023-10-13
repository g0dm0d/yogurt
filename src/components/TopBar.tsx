/* eslint-disable react/react-in-jsx-scope */
/* eslint-disable @typescript-eslint/explicit-function-return-type */
import {
  Box
} from '@mantine/core'
import { IconBrowser } from '@tabler/icons-react'

export function TopBar () {
  return (
        <Box sx={(theme) => ({
          display: 'flex',
          height: '30px',
          paddingRight: '8px',
          gap: '8px',
          justifyContent: 'end',
          alignItems: 'center',
          backgroundColor: theme.colors.dark[7]
        })}>
            <IconBrowser color='white' stroke={1.5} />
            <IconBrowser color='white' stroke={1.5} />
            <IconBrowser color='white' stroke={1.5} />
        </Box>
  )
}
