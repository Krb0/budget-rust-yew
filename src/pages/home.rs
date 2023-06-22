use yew::{function_component, html, Html};
use yew_router::prelude::use_navigator;

use crate::layout::Layout;
use crate::utils::navigator_redirect;
use crate::pages::Route;
#[function_component(Home)]
pub fn home() -> Html {
     let navigator = use_navigator().unwrap();
     
     let redirect_register = navigator_redirect(navigator, Route::Register);
    html! {
        <div>
            <Layout>
                <div class="flex justify-center " style="gap:45px">
                    <div class="lg:2/6 xl:w-2/4 mt-20 lg:mt-40 lg:ml-16 text-left">
                        <div class="text-6xl font-semibold text-gray-900 leading-none">
                            {"Take your finances to next level"}
                        </div>
                        <div class="mt-6 text-xl font-light text-true-gray-500 antialiased">
                            {"Manage your money like a pro!"}
                        </div>
                        <button onclick={redirect_register} class="mt-6 px-8 py-4 rounded-full font-normal tracking-wide bg-gradient-to-b from-blue-600 to-blue-700 text-white outline-none focus:outline-none hover:shadow-lg transition duration-200 ease-in-out">
                            {"Sign Up"}
                        </button>
                    </div>
                    <SavingImage/>
                </div>
            </Layout>
        </div>
    }
}



#[function_component(SavingImage)]
fn saving_image() -> Html{

    html!{
        <svg xmlns="http://www.w3.org/2000/svg" style="margin-top:4rem" data-name="Layer 1" width="550" viewBox="0 0 740.67538 597.17519"><path d="M651.61162,679.71942c-5.12158-7.17175-12.05469-12.41919-20.771-14.38311-7.60058-1.71259-15.416-1.42072-23.14209-1.36115l-97.69238.7533c-8.3667.06451-16.38867,7.31311-16,16,.38574,8.61358,7.03174,16.06915,16,16q42.92578-.331,85.85108-.662c6.74658-.052,13.49707-.17371,20.24414-.15155,1.51513.00494,3.02392.08484,4.53759.102.3667.00415.63135.00091.81836-.007.04688.01428.08643.02734.14746.04407.78956.21679,1.59961.36621,2.394.57214-.27832.0083.51513.5033.91357.671.05469.108.17139.29938.36621.59515.272.51,1.07715,2.05866,1.11035,2.00989a71.76183,71.76183,0,0,1,2.98682,9.39289c1.85938,7.04718,2.26807,8.52026,2.53565,15.13195.33837,8.352,7.11718,16.39783,16,16,8.3789-.3753,16.36279-7.04236,16-16C663.3001,709.33283,660.65068,692.37738,651.61162,679.71942Z" transform="translate(-229.66231 -151.4124)" fill="#e6e6e6"/><path d="M515.74883,715.82494c-1.835-11.11963-3.63477-22.24637-5.50537-33.35952-1.898-11.27417-5.77442-21.59812-14.54786-28.9975-2.34423-1.97688-4.7749-3.82268-7.21582-5.66276-.85839-.6468-1.71533-1.29456-2.56347-1.95566-.05078-.03952-.083-.0643-.127-.09861-.09326-.0802-.20507-.17551-.35546-.30272-3.65674-3.0948-7.19-6.32723-10.55567-9.76318q-4.48315-4.57729-8.57471-9.545-1.02246-1.24071-2.01855-2.50366c-.11426-.14481-.72949-.93992-1.13477-1.45875-.377-.51927-1.06591-1.457-1.18164-1.62073q-1.1638-1.64569-2.28613-3.322a156.12618,156.12618,0,0,1-13.187-24.11549q-.54713-1.25119-1.06836-2.51707c-.09326-.25754-.21387-.58834-.374-1.02876-.74121-2.03833-1.481-4.07418-2.15869-6.13665q-2.15992-6.57108-3.71387-13.3362-.76832-3.34049-1.3833-6.7149-.31715-1.73529-.59326-3.47777c-.03028-.19285-.21-1.44617-.31592-2.15436-.07813-.71162-.22364-1.97155-.24317-2.16618q-.17577-1.75646-.312-3.51634-.30688-3.96012-.415-7.93346-.18456-7.06137.25293-14.12171c.166-2.64466.40235-5.28245.69336-7.91529.01758-.16178.03125-.29046.04395-.40642.01758-.11756.03808-.25112.0625-.41735.21533-1.455.44189-2.908.69531-4.35644a159.29393,159.29393,0,0,1,3.73926-16.31344q1.08765-3.80858,2.36279-7.55538.56836-1.66325,1.17285-3.31377c-.02637.071,1.18213-3.01574,1.19824-3.11945,4.79493-11.32906,12.6377-21.657,12.64844-34.52643.00733-8.71332-7.36426-17.06006-16-16.65739a16.62541,16.62541,0,0,0-16,16.65739c0,.19565-.001.36258-.00146.52461-.08057.27127-.14795.49761-.2085.69777-.93506,2.032-2.08057,3.96285-3.10693,5.94438-2.81055,5.42345-5.20948,11.02482-7.48828,16.70842a159.82519,159.82519,0,0,0-8.5293,28.62825,182.098,182.098,0,0,0-.96777,65.04019c6.20361,37.8128,26.66162,74.009,55.103,98.33928,2.46826,2.11134,4.99658,4.15171,7.54883,6.15089,1.05615.82714,2.12109,1.63909,3.19141,2.44577.50586.38139,2.60156,2.034,1.67968,1.23331,1.98243,1.69183,2.8169,2.5433,3.32618,3.78944.07373.2876.17431.67762.31054,1.21088.35889,1.40538.69971,2.80751.98291,4.23259,2.47559,12.45143,4.19629,25.11159,6.26416,37.64359,1.42139,8.61064,12.022,14.21487,19.68213,11.6342C513.49,733.31151,517.27178,725.05119,515.74883,715.82494Z" transform="translate(-229.66231 -151.4124)" fill="#e6e6e6"/><path d="M653.23,748.12012a25.37427,25.37427,0,0,1-25.43994-25.25V704.61865c-19.10889,2.13916-84.54321,1.64356-103.29981-1.37011v18.8916a25.4409,25.4409,0,0,1-50.88037,0v-37.0918a163.30721,163.30721,0,0,1-84-142.38818c0-49.83008,22.583-96.31446,61.95874-127.5337a164.07476,164.07476,0,0,1,89.32251-35.17333c4.427-.34717,8.88086-.52295,13.23877-.52295,15.3523,0,84.61866,2.37158,101.99781,7.37158L695.75562,347.48a4.16,4.16,0,0,1,4.55-.90674,4.10044,4.10044,0,0,1,2.58448,3.84668v6.252l7.71631-7.64209a4.1505,4.1505,0,0,1,4.54638-.90821,4.10219,4.10219,0,0,1,2.5874,3.84815v69.86474a162.75915,162.75915,0,0,1-39.07031,267.86621v33.16944A25.37427,25.37427,0,0,1,653.23,748.12012ZM629.79,702.35059v20.51953a23.44072,23.44072,0,0,0,46.87988,0V688.4375l.571-.271a162.45628,162.45628,0,0,0,66.53076-58.18164,160.32526,160.32526,0,0,0-27.69922-206.96l-.33228-.29834V351.96973a2.10314,2.10314,0,0,0-1.34179-1.99561,2.1492,2.1492,0,0,0-2.38135.47315l-11.127,11.0205V350.41992a2.10314,2.10314,0,0,0-1.3418-1.9956,2.15435,2.15435,0,0,0-2.38525.47656l-40.47632,40.16357-.5752-.18017c-16.156-5.05664-86.51269-7.4541-101.98169-7.4541-4.30591,0-8.70727.17382-13.08154.51709a162.07488,162.07488,0,0,0-88.23682,34.74609c-38.89453,30.83789-61.20166,76.751-61.20166,125.9668A161.31464,161.31464,0,0,0,475.094,683.585l.51587.28565v38.26953a23.441,23.441,0,0,0,46.88037,0V700.85449l1.18482.22266c16.85571,3.17139,87.83179,3.67139,104.98486,1.42139Z" transform="translate(-229.66231 -151.4124)" fill="#3f3d56"/><path d="M517.66016,270.53,406.91992,303.81a13.8823,13.8823,0,0,0,7.99024,26.59l96.90966-29.13,2.03028-.61005a67.51666,67.51666,0,0,1,8.29-29.86,6.16489,6.16489,0,0,0-2-.52A6.367,6.367,0,0,0,517.66016,270.53Z" transform="translate(-229.66231 -151.4124)" fill="#1d6ed1"/><path d="M412.995,337.49524a7.01287,7.01287,0,0,1-6.63781-4.80127l-6.92481-20.896a6.99947,6.99947,0,0,1,4.44324-8.84668l90.13122-29.87012a6.99966,6.99966,0,0,1,8.84668,4.44189l6.92481,20.896a6.97565,6.97565,0,0,1-4.19312,8.75928q-.12487.04688-.25219.08838l-90.12915,29.86914A7.003,7.003,0,0,1,412.995,337.49524Z" transform="translate(-229.66231 -151.4124)" fill="#2f2e41"/><path d="M581.79,233.42a70.03128,70.03128,0,0,0-69.97022,67.85c-.02.71-.02978,1.43-.02978,2.15a70,70,0,1,0,70-70Zm0,138a68.071,68.071,0,0,1-68-68q0-1.395.06006-2.76A67.998,67.998,0,1,1,581.79,371.42Z" transform="translate(-229.66231 -151.4124)" fill="#3f3d56"/><path d="M640.79,303.42a59,59,0,0,1-118,0c0-1.87.08984-3.71.25976-5.53A59,59,0,0,1,640.79,303.42Z" transform="translate(-229.66231 -151.4124)" fill="#e6e6e6"/><path d="M599.35986,307.34a14.04753,14.04753,0,0,0-5.46972-4.33,53.13872,53.13872,0,0,0-8.22022-2.7V284.25a23.39587,23.39587,0,0,1,11.68994,5.33l3.26026-6.95a21.56068,21.56068,0,0,0-6.36035-3.93,31.25931,31.25931,0,0,0-8.06983-2.07v-8.14h-7.5498v8.22a20.67243,20.67243,0,0,0-8.25,2.77,16.3988,16.3988,0,0,0-5.59034,5.48,13.77113,13.77113,0,0,0-1.98974,7.29,12.46275,12.46275,0,0,0,2.29,7.8,14.51022,14.51022,0,0,0,5.6997,4.52,52.69206,52.69206,0,0,0,8.36036,2.77v15.39A24.98812,24.98812,0,0,1,565.25,317.18l-3.25977,6.96a24.58042,24.58042,0,0,0,7.43995,4.22,34.16142,34.16142,0,0,0,9.21,1.92v8.07h7.5498v-8.22a19.5385,19.5385,0,0,0,11.21-5.14,13.58959,13.58959,0,0,0,4.18018-10.1A12.0549,12.0549,0,0,0,599.35986,307.34Zm-20.1997-8.73a14.49857,14.49857,0,0,1-5.03028-2.7,5.32046,5.32046,0,0,1-1.77978-4.18,6.72557,6.72557,0,0,1,1.81982-4.81,9.20221,9.20221,0,0,1,4.99024-2.59Zm11.25,21.28a8.5575,8.5575,0,0,1-4.74024,2.47V309.12a13.23964,13.23964,0,0,1,4.74024,2.37,4.65775,4.65775,0,0,1,1.62988,3.7A6.8039,6.8039,0,0,1,590.41016,319.89Z" transform="translate(-229.66231 -151.4124)" fill="#1d6ed1"/><path d="M416.209,422.18421a7.02318,7.02318,0,0,1-6.775-5.21582l-3.77014-14.27a7.0039,7.0039,0,0,1,4.97509-8.55127l118.94068-31.42969a7.01537,7.01537,0,0,1,8.551,4.98535l3.19385,12.08984-.59839.04639a160.16254,160.16254,0,0,0-84.10754,31.8711l-.17346.084-38.46021,10.15967A6.96258,6.96258,0,0,1,416.209,422.18421Z" transform="translate(-229.66231 -151.4124)" fill="#2f2e41"/><path d="M396.51011,492.6705a98.108,98.108,0,0,1-39.12217-14.79216A95.527,95.527,0,0,1,341.214,464.65c-4.82786-4.92434-9.4465-10.52122-11.83781-17.0792a25.299,25.299,0,0,1,.33124-18.7693,24.33352,24.33352,0,0,1,13.16876-13.12553c5.98961-2.38861,13.22619-2.64412,19.05541-.02821a17.08926,17.08926,0,0,1,6.96433,5.60584,11.43493,11.43493,0,0,1,1.8103,3.75816,7.67372,7.67372,0,0,1,.09036,3.75911,9.01634,9.01634,0,0,1-4.77388,5.723,24.06819,24.06819,0,0,1-8.1521,2.48626c-2.86013.39-6.96868.74485-9.268-1.551a4.66934,4.66934,0,0,1-1.09261-1.61079,3.1144,3.1144,0,0,1-.17238-1.45807c-.03354.21724.13872-.47236.12891-.44486-.03636.10194-.19384.3504.03-.03135a3.39683,3.39683,0,0,1,.23311-.363c-.076.09827-.24291.26227.05515-.01912a1.96186,1.96186,0,0,1,.91427-.56441c.13841-.04032.27883-.07242.41763-.11033.35277-.09637-.35161.00526.0093-.00631.13927-.00446.28021-.001.419-.00726.354-.01591-.34927-.09655-.01-.01126.13173.03312.26222.06547.392.10635.28208.08887-.19974-.20693-.00859.01446.224.25945-.23521-.25235.0194-.00119a4.6244,4.6244,0,0,1,.35767.39431,2.35924,2.35924,0,0,0,3.315,0,2.39935,2.39935,0,0,0,0-3.315c-2.3114-2.86535-6.87707-2.15735-9.21528.26436-2.982,3.08848-1.80111,7.98318,1.09954,10.678,3.2837,3.05076,7.9344,3.26284,12.15834,2.75114a29.817,29.817,0,0,0,10.26918-2.902,14.39829,14.39829,0,0,0,7.024-7.58817,13.17381,13.17381,0,0,0-1.05641-10.76889c-3.7293-7.01118-11.45152-10.60575-19.08437-11.22837a29.81221,29.81221,0,0,0-21.81879,7.16278,30.15422,30.15422,0,0,0-9.88822,20.075c-.66626,7.996,2.43608,15.76128,6.95922,22.21432a84.62746,84.62746,0,0,0,16.77944,17.3533,102.47323,102.47323,0,0,0,42.812,20.19071q2.79237.57685,5.61474.98758a2.42141,2.42141,0,0,0,2.88342-1.63715,2.36125,2.36125,0,0,0-1.63716-2.88343Z" transform="translate(-229.66231 -151.4124)" fill="#3f3d56"/><path d="M619.8954,390.79425q-30.6631-.43278-61.133,1.55634c-6.9433.45338-12.95072,2.90742-12.95072,7.03206,0,3.44855,5.95964,7.48858,12.95072,7.032q30.45443-1.9887,61.133-1.55633C636.56931,405.09367,636.55948,391.02941,619.8954,390.79425Z" transform="translate(-229.66231 -151.4124)" fill="#ccc"/><path d="M589.68169,404.16175c-2.20923,0-54.10571-.08252-54.10571-7.251,0-7.168,51.89648-7.25049,54.10571-7.25049s54.10571.08252,54.10571,7.25049C643.7874,404.07923,591.89092,404.16175,589.68169,404.16175Zm-52.07642-7.251c1.42017,2.17334,20.49634,5.251,52.07642,5.251s50.656-3.07764,52.07641-5.251c-1.42016-2.17285-20.49633-5.25049-52.07641-5.25049S539.02544,394.73792,537.60527,396.91077Z" transform="translate(-229.66231 -151.4124)" fill="#3f3d56"/><path d="M754.33555,592.61209c11.48831-.319,25.78612-.71776,36.801-8.76407a28.26188,28.26188,0,0,0,11.12052-21.10332c.22483-6.35459-2.071-11.90149-6.46488-15.61408-5.75306-4.86143-14.154-6.00169-23.20738-3.34108l9.38-68.548-6.88568-.94343L764.052,554.885l5.75009-2.63856c6.66637-3.05766,15.81731-4.61365,21.50378.19174a12.21541,12.21541,0,0,1,4.00576,10.06213,21.36089,21.36089,0,0,1-8.275,15.7346c-8.5719,6.26126-19.96858,7.06894-32.89451,7.43036Z" transform="translate(-229.66231 -151.4124)" fill="#3f3d56"/><rect x="468.20272" y="329.7408" width="37.43223" height="6.95017" fill="#3f3d56"/><path d="M433.14751,454.15a7.01243,7.01243,0,0,1-6.904-5.91748L423.95867,433.651a7.00763,7.00763,0,0,1,5.83215-7.99951L551.32231,406.609a7.0074,7.0074,0,0,1,7.999,5.832l2.28467,14.582a7.00726,7.00726,0,0,1-5.83179,7.99952l-121.5315,19.042A7.09551,7.09551,0,0,1,433.14751,454.15Z" transform="translate(-229.66231 -151.4124)" fill="#2f2e41"/><circle cx="153.18938" cy="80.85472" r="51" fill="#1d6ed1"/><path d="M411.727,245.46306a12.09515,12.09515,0,0,0,4.36685-2.67614,8.13345,8.13345,0,0,0,2.25475-6.483,5.47156,5.47156,0,0,0-2.51209-4.16412c-1.8457-1.13505-4.28459-1.09217-6.74692.0493l-.2842-19.90787-1.99969.02846.334,23.40381,1.52231-.99844c1.76529-1.15588,4.30177-1.99407,6.12663-.87157a3.51373,3.51373,0,0,1,1.57291,2.68953,6.14619,6.14619,0,0,1-1.67626,4.83366c-2.16794,2.15111-5.37684,2.87189-9.03924,3.53112l.35442,1.96814A32.96256,32.96256,0,0,0,411.727,245.46306Z" transform="translate(-229.66231 -151.4124)" fill="#2f2e41"/><rect x="418.83797" y="211.67905" width="10.77167" height="1.99975" transform="translate(-256.7253 -85.49311) rotate(-8.61162)" fill="#2f2e41"/><rect x="385.22114" y="216.76898" width="10.77167" height="1.99975" transform="translate(-257.86644 -90.46937) rotate(-8.61162)" fill="#2f2e41"/><path d="M416.55474,453.25843a9.03041,9.03041,0,0,1-8.43445-5.855L372.9572,353.2345a46.53156,46.53156,0,0,1,27.28418-59.80225L428.83306,282.756a8.95769,8.95769,0,0,1,6.87805.23877,8.83186,8.83186,0,0,1,4.67541,4.98584L505.523,410.96448a9.0015,9.0015,0,0,1-5.25659,11.63818L419.7,452.68665A8.97612,8.97612,0,0,1,416.55474,453.25843Z" transform="translate(-229.66231 -151.4124)" fill="#2f2e41"/><path d="M420.15839,285.99487l64.629,86.42566L441.05579,286.902a8.079,8.079,0,0,0-10.349-4.846Z" transform="translate(-229.66231 -151.4124)" fill="#e6e6e6"/><path d="M522.49036,351.63l-115.39525-7.43312a13.87944,13.87944,0,1,1,1.78425-27.70148l115.3953,7.43258a6.49946,6.49946,0,0,1,6.06882,6.90407l-.9485,14.72919a6.50056,6.50056,0,0,1-6.90462,6.06876Z" transform="translate(-229.66231 -151.4124)" fill="#1d6ed1"/><path d="M491.65508,356.39514q-.22852,0-.45825-.01464h-.00025l-94.7561-6.10352a6.99919,6.99919,0,0,1-6.53565-7.43506l1.62036-25.1582a6.99967,6.99967,0,0,1,7.43592-6.53516l94.756,6.103a7.00755,7.00755,0,0,1,6.5354,7.43506l-1.62012,25.15869a7.01886,7.01886,0,0,1-4.54614,6.11084A6.926,6.926,0,0,1,491.65508,356.39514Z" transform="translate(-229.66231 -151.4124)" fill="#2f2e41"/><path d="M382.26706,198.35294c6.70923-5.77031,15.92258-.60839,23.61673-1.512,7.36114-.86455,13.17369-7.46753,14.65857-14.48237,1.73235-8.184-2.51553-16.38319-8.77663-21.48577-6.85745-5.58858-15.95214-7.19293-24.57042-6.159-9.87767,1.185-18.91123,5.78935-27.017,11.35854a121.85037,121.85037,0,0,0-21.44164,18.16917c-5.75111,6.35166-10.62377,13.9227-12.17974,22.46976-1.414,7.76751-.33431,16.375,4.2575,22.95514a24.20528,24.20528,0,0,0,9.47417,7.80529c3.93743,1.93344,8.13754,3.31565,11.95217,5.50169,5.76775,3.30532,11.37351,10.15883,9.56677,17.29094a9.79281,9.79281,0,0,1-2.226,4.19c-1.29065,1.43275-3.6187-.46055-2.32465-1.89708,2.27282-2.52305,2.18331-5.88792.97206-8.90023a16.27233,16.27233,0,0,0-7.23346-7.95427c-3.993-2.28294-8.40146-3.6921-12.494-5.77156a27.02254,27.02254,0,0,1-9.91143-8.21385c-4.89822-6.807-6.36338-15.66585-5.2525-23.861,1.20214-8.86841,5.75869-16.9272,11.47065-23.68045,6.21621-7.34944,13.8519-13.57054,21.65246-19.15936,8.37006-5.99685,17.61734-11.06678,27.85008-12.90049,8.87016-1.58954,18.48474-.61091,26.24132,4.22294,7.24026,4.51209,12.79232,12.27377,13.34449,20.9467a22.23537,22.23537,0,0,1-10.37627,19.94206,19.101,19.101,0,0,1-11.41,2.64333c-4.2616-.27279-8.52728-1.57778-12.81609-1.21548a8.94077,8.94077,0,0,0-5.26807,2.1285c-1.46588,1.26074-3.21292-1.1807-1.75908-2.43109Z" transform="translate(-229.66231 -151.4124)" fill="#2f2e41"/></svg>
    }
}