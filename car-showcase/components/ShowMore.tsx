'use client';

import { ShowMoreProps } from '@/types';
import { useRouter } from 'next/navigation';
import CustomButton from './CustomButton';
import { updateSearchParams } from '@/utils';

const ShowMore = ({ pageNumber, isNextPage, setLimit }: ShowMoreProps) => {
  const handdleNavigation = () => {
    const newLimit = (pageNumber + 1) * 10;
    setLimit(newLimit);
  };
  return (
    <div className="w-full flex-center gap-5 mt-10">
      {!isNextPage && (
        <CustomButton
          title="Show More"
          btnType="button"
          textStyles="text-white font-bold"
          containerStyles="bg-primary-blue rounded-full"
          handleClick={handdleNavigation}
        />
      )}
    </div>
  );
};

export default ShowMore;
