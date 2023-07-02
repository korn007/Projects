'use client';
import { CarCard, CustomFilter, Hero, SearchBar, ShowMore } from '@/components';
import { fuels, yearsOfProduction } from '@/constants';
import { fetchCars } from '@/utils';
import Image from 'next/image';
import { useEffect, useState } from 'react';

export default function Home() {
  const [allCars, setAllCars] = useState([]);
  const [isLoading, setIsLoading] = useState(false);

  // Serach state
  const [manufacturer, setManufacturer] = useState('bmw');
  const [model, setModel] = useState('');

  // Filter state
  const [fuel, setFuel] = useState('');
  const [year, setYear] = useState(2022);

  //Pagination state
  const [limit, setLimit] = useState(10);

  const fetchAllCars = async () => {
    setIsLoading(true);
    try {
      const result = await fetchCars({
        manufacturer,
        model,
        year,
        fuel,
        limit,
      });
      setAllCars(result);
      setIsLoading(false);
    } catch (error) {
      console.log(error);
    } finally {
      setIsLoading(false);
    }
  };

  useEffect(() => {
    console.log('fetching...');
    console.log(fuel, year, limit, manufacturer, model);
    fetchAllCars();
  }, [manufacturer, model, year, fuel, limit]);
  //const searchParams = useSearchParams();

  // const allCars = await fetchCars({
  //   manufacturer: searchParams.get('manufacturer') || 'bmw',
  //   model: searchParams.get('model') || '',
  //   year: searchParams.get('year') || '',
  //   fuel: searchParams.get('fuel') || '',
  //   limit: searchParams.get('limit') || '10',
  // });
  const isDataEmpty = !Array.isArray(allCars) || allCars.length < 1 || !allCars;
  return (
    <main className="overflow-hidden">
      <Hero />
      <div className="mt-12 padding-x padding-y max-width" id="discover">
        <div className="home__text-container">
          <h1 className="text-4xl font-extrabold">Car Catalogue</h1>
          <p>Explore the car you might like.</p>
        </div>
        <div className="home__filters">
          <SearchBar setManufacturer={setManufacturer} setModel={setModel} />
          <div className="home__filter-container">
            <CustomFilter title="fuel" options={fuels} setFilter={setFuel} />
            <CustomFilter
              title="year"
              options={yearsOfProduction}
              setFilter={setYear}
            />
          </div>
        </div>
        {allCars.length > 0 ? (
          <section>
            <div className="home__cars-wrapper">
              {allCars?.map((car) => (
                // eslint-disable-next-line react/jsx-key
                <CarCard car={car} />
              ))}
            </div>
            {isLoading && (
              <div className="mt-16 w-full flex-center">
                <Image
                  src="/Spinner-1s-200px.svg"
                  width={100}
                  height={100}
                  alt="loader"
                  className="object-contain"
                />
              </div>
            )}
            <ShowMore
              pageNumber={limit / 10}
              isNextPage={limit > allCars.length}
              setLimit={setLimit}
            />
          </section>
        ) : (
          <div className="home__error-container">
            <h2 className="text-black text-xl font-bold">Oops, no results</h2>
            <p>{allCars?.message}</p>
          </div>
        )}
      </div>
    </main>
  );
}
