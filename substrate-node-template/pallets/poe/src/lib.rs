
#![cfg_attr(not(feature = "std"), no_std)]  //编译标签
// 模块源代码文件
pub use pallet::*;    //把引用类型暴露出来

//test zhoufy 2021-12-26

//test 
//step 1 copy template\src\mock.rs、tests.rs
//step 2 edit template to poe,remove default test case
//step 3 add mod ref mock/test in lib.rs
//step 4 cmd: cargo -- test 运行测试用例，-p标签指定测试模块 
/*
  cargo -- test -p pallet_poe
  运行结果 test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
*/
//step 5 完善测试用例tests.rs

#[cfg(test)]    //因为是测试，增加测试标签 
mod mock;      //测试runtime，拷贝模板下的mock.rs模块后(修改template为poe)，引入模块
#[cfg(test)]
mod tests;      //测试用例模块，拷贝模板下的tests.rs模块后，引入模块

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

    //zhoufy 2021-12-26 作业 创建存证检查长度，在错误枚举中增加CheckLenthShort/CheckLenthLong
    CheckLenthShort,
		CheckLenthLong,
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
      
        let checklen = true;
        
        //ensure!(!Proofs::<T>::contains_key(&proof), Error::<T>::ProofAlreadyClaimed);        
        {
          //zhoufy 2021-12-26 作业 创建存证检查长度
          ensure!(proof.len() >= 0 as usize, Error::<T>::CheckLenthShort);
          ensure!(proof.len() <= 65536 as usize, Error::<T>::CheckLenthLong);
    
          // Verify that the specified proof has not already been claimed.
          ensure!(!Proofs::<T>::contains_key(&proof), Error::<T>::ProofAlreadyClaimed);
        }

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

        
      //转移存证
      #[pallet::weight(0)]
      pub fn transfer_claim(origin: OriginFor<T>, proof: Vec<u8>, dest: T::AccountId) ->
      // DispatchResult {
      DispatchResultWithPostInfo {
        let sender = ensure_signed(origin)?;
        ensure!(Proofs::<T>::contains_key(&proof), Error::<T>::NoSuchProof);

        let (owner, _) = Proofs::<T>::get(&proof).ok_or(Error::<T>::NoSuchProof)?;

        ensure!(owner == sender, Error::<T>::NotProofOwner);

        let current_block = <frame_system::Pallet<T>>::block_number();

        Proofs::<T>::insert(&proof, (&dest, current_block));

        Self::deposit_event(Event::ClaimTransfered(sender,dest,proof));
        //Ok(())
        Ok(().into())
    }
    
     
  }
}