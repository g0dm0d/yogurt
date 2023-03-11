import { createContext, SetStateAction } from 'react';

interface selectedAccountProps {
    nickname: string;
    changeNickname: (value: string) => void;
}

export const selectedAccount = createContext<Partial<selectedAccountProps>>({});