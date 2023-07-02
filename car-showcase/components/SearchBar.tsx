'use client';

import { useState } from 'react';
import SearchManufacturer from './SearchManufacturer';
import Image from 'next/image';
import { useRouter } from 'next/navigation';
interface SearchBarProps {
  setManufacturer: (manufacturer: string) => void;
  setModel: (model: string) => void;
}
const SearchBar = ({ setManufacturer, setModel }: SearchBarProps) => {
  const router = useRouter();
  const [searchManufacturer, setSearchManufacturer] = useState('');
  const [searchModel, setSearchModel] = useState('');
  const handleSearch = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    console.log('searching...');
    if (searchManufacturer === '' && searchModel === '') {
      return alert('Please enter a manufacturer or model');
    }
    setManufacturer(searchManufacturer);
    setModel(searchModel);
  };

  const SearchButton = ({ otherClasses }: { otherClasses: string }) => (
    <button className={`-ml-3 z-10 ${otherClasses}`} type="submit">
      <Image
        src="/magnifying-glass.svg"
        alt="smagnifying glass"
        width={40}
        height={40}
      />
    </button>
  );
  return (
    <form className="searchbar" onSubmit={handleSearch}>
      <div className="searchbar__item">
        <SearchManufacturer
          manufacturer={searchManufacturer}
          setMenuFacturer={setSearchManufacturer}
        />
        <SearchButton otherClasses="sm:hidden" />
      </div>
      <div className="searchbar__item">
        <Image
          src="/model-icon.png"
          alt="car model"
          width={25}
          height={25}
          className="absolute w-[20px] h-[20px] ml-4"
        />
        <input
          type="text"
          placeholder="Tigan"
          className="searchbar__input"
          value={searchModel}
          onChange={(e) => setSearchModel(e.target.value)}
        />
        <SearchButton otherClasses="sm:hidden" />
      </div>
      <SearchButton otherClasses="max-sm:hidden" />
    </form>
  );
};

export default SearchBar;
