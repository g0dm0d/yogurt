/* eslint-disable react/react-in-jsx-scope */
/* eslint-disable @typescript-eslint/explicit-function-return-type */
import * as core from '@mantine/core'
import { SettingsDashboard } from './SettingsDashboard'

export function InstanceSettings () {
  return (
        <core.Box sx={{ display: 'flex', alignItems: 'center', minHeight: '30vh', width: '80wh' }}>
            <SettingsDashboard />
        </core.Box>
  )
}
