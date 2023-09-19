import { useState, useEffect, useCallback, useMemo } from 'react';
import { Reveal } from 'react-awesome-reveal';
import { useNavigate } from '@reach/router';
import { toast} from 'react-toastify';
import { fadeInUp } from '../../utils';
import { useCustomWallet } from '../../context/WalletContext';
import useBounty from '../../hooks/useBounty';
import useBackend from '../../hooks/useBackend';

export const MyBountyBodyListItem = ({ bountyId }) => {
  const { isConnected, walletAddress } = useCustomWallet();
  const { closeBounty } = useBounty();
  const { getSingleBounty, countSubmissions, closeBountyB } = useBackend();

  const [bounty, setBounty] = useState([]);
  const [submissions, setSubmissions] = useState(0);
  
  const isExpired = useMemo(() => bounty?.endDate <= Date.now(), [bounty]);

  const onClickClaim = useCallback(async(event) => {
    if (!isConnected) {
      toast.warning('Wallet not connected yet!');
      return;
    }

    const res1 = await closeBounty(walletAddress, bountyId);
    if (res1) {
      const error = await getLastError();
      toast.error('Failed to close to bounty!');
      console.error('error:', error);
      return;
    }

    const res2 = await closeBountyB(walletAddress, bountyId);
    if (res2) {
      toast.error('Failed to close bounty!');
      return;
    }

    toast('Successfully closed bounty!');
  }, [isConnected, walletAddress, bountyId]);

  useEffect(() => {
    async function fetchBounties() {
      if (!isConnected || !walletAddress || !bountyId)
        return;

      const singleBounty = await getSingleBounty(bountyId);
      // console.log('singleBounty:', singleBounty);
      setBounty(singleBounty);

      const submitCount = await countSubmissions(walletAddress, bountyId);
      // console.log('submitCount:', submitCount);
      setSubmissions(submitCount);
    }

    fetchBounties();
  }, [isConnected, walletAddress, bountyId]);

  const nav = useNavigate();

  return (
    <div className='app-body'>
      <Reveal keyframes={fadeInUp} className='onStep' delay={400} duration={1000} triggerOnce>
        <div className='row'>
          <div className='w-full lg:pl-0 mt-[20px] pr-0'>
            <div className='app-card cursor-pointer'>
              <div className='flex justify-between sm:flex-col sm:text-center sm:items-center sm:gap-3'>
                  <div className='text-[16px]'>{bounty?.title}</div>
                  <div className='flex flex-row space-x-2'>
                    {
                      isExpired ? 
                      (<div className='flex-col justify-around space-x-2  sm:flex-col sm:text-center border rounded-2xl px-4 relative' onClick={onClickClaim}>
                        Claim
                      </div>)
                       :
                      (submissions > 0 ?
                        (<div className='flex-col justify-around space-x-2  sm:flex-col sm:text-center border rounded-2xl px-4 relative' onClick={()=>nav('/MyBounties/' + bountyId)}>
                          Review
                          <div className='my-badge'> {submissions} </div>
                        </div>)
                         :
                        (<div className='flex-col justify-around space-x-2  sm:flex-col sm:text-center px-1'>No Submissions</div>)
                      )
                    }
                  </div>
              </div>

            </div>
          </div>
        </div>
      </Reveal>
    </div>
  );
}