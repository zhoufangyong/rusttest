
#![cfg_attr(not(feature = "std"), no_std)]  //编译标签
// 模块源代码文件
pub use pallet::*;    //把引用类型暴露出来

///存证相关的功能模块


#[frame_support::pallet]
pub mod pallet {
  use frame_support::{
      dispatch::DispatchResultWithPostInfo,
      pallet_prelude::*
    };
  use frame_system::pallet_prelude::*;
  use sp_std::vec::Vec; 

  
  /// 模块配置接口
  #[pallet::config]
  pub trait Config: frame_system::Config {
  type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
  }

  ///Pallet结构体承载功能模块，依赖存储单元
  #[pallet::pallet]
  #[pallet::generate_store(pub(super) trait Store)]
  pub struct Pallet<T>(_);

  ///定义存储单元模块，只有一个Proofs存储存证，类型StorageMap
  #[pallet::storage]
  #[pallet::getter(fn proofs)]
  pub type Proofs<T:Config> = StorageMap<
    _,
    Blake2_128Concat,
    Vec<u8>,                        //key
    (T::AccountId,T::BlockNumber)   //value,用户ID及存入存证区块，来自系统模块
  >;

  #[pallet::event]
  #[pallet::generate_deposit(pub(super) fn deposit_event)]
  pub enum Event<T: Config> {
    //创建存证触发的事件
    ClaimCreated(T::AccountId, Vec<u8>),
    //撤销存证触发的事件
    ClaimRevoked(T::AccountId, Vec<u8>),
    //转移存证触发的事件
    ClaimTransfered(T::AccountId, T::AccountId, Vec<u8>),
  }

  #[pallet::error]
  pub enum Error<T> {
    ProofAlreadyClaimed,
    NoSuchProof,
    NotProofOwner,
    }


    //空的函数，预留
  #[pallet::hooks]
  impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

///可调用函数
#[pallet::call]
impl<T: Config> Pallet<T> {
  //创建存证
  #[pallet::weight(1000)]
  pub fn create_claim(
    origin: OriginFor<T>,   //交易发送方
    proof: Vec<u8>,         //存证hash值
    //) -> DispatchResult {
    ) -> DispatchResultWithPostInfo {

      //逻辑： 校验发送方签名、
        let sender = ensure_signed(origin)?;
      ensure!(!Proofs::<T>::contains_key(&proof), Error::<T>::ProofAlreadyClaimed);
      let current_block = <frame_system::Pallet<T>>::block_number();
      // 存储发送方块
      Proofs::<T>::insert(&proof, (&sender, current_block)); //key value
      //出发事件
      Self::deposit_event(Event::ClaimCreated(sender, proof));
      //Ok(())
      Ok(().into())
      }
      //撤销存证
      #[pallet::weight(1001)]
      pub fn revoke_claim(
        origin: OriginFor<T>,
        proof: Vec<u8>,
        //) -> DispatchResult {
        ) -> DispatchResultWithPostInfo {
          //校验
          let sender = ensure_signed(origin)?;

          //存储里是否有，  15:49
          //ensure!(Proofs::<T>::contains_key(&proof), Error::<T>::NoSuchProof);          
          let (owner, _) = Proofs::<T>::get(&proof).ok_or(Error::<T>::NoSuchProof)?;

          
          ensure!(sender == owner, Error::<T>::NotProofOwner);

          // 调用，从存储里删除
          Proofs::<T>::remove(&proof);

          // Emit an event that the claim was erased.
          Self::deposit_event(Event::ClaimRevoked(sender, proof));
          //Ok(())
          Ok(().into())
        }

        /*
      //转移存证
      #[pallet::weight(0)]
      pub fn transfer_claim(origin: OriginFor<T>, proof: Vec<u8>, dest: T::AccountId) ->
      // DispatchResult {
      DispatchResultWithPostInfo {
        let sender = ensure_signed(origin)?;
        ensure!(Proofs::<T>::contains_key(&proof), Error::<T>::NoSuchProof);

        let (owner, _block_number) = Proofs::<T>::get(&proof);

        ensure!(owner == sender, Error::<T>::NotProofOwner);

        let current_block = <frame_system::Pallet<T>>::block_number();

        Proofs::<T>::insert(&proof, (&dest, current_block));

        Self::deposit_event(Event::ClaimTransfered(sender,dest,proof));
        //Ok(())
        Ok(().into())
    }
    */
     
  }
}