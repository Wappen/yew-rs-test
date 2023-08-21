use crate::model::ArticleModel;
use chrono::Utc;

pub trait ArticleRepository {
    fn find_all(&self) -> Vec<ArticleModel>;
    fn find_by_id(&self, id: i32) -> Option<ArticleModel>;
}

impl PartialEq for dyn ArticleRepository {
    fn eq(&self, other: &Self) -> bool {
        self.find_all() == other.find_all()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct MockArticleRepository {
    articles: Vec<ArticleModel>,
}

impl Default for MockArticleRepository {
    fn default() -> Self {
        MockArticleRepository {
            articles: vec![
                ArticleModel {
                    id: 1,
                    title: "Hello World!".to_string(),
                    author: "Lenz".to_string(),
                    description: "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.".to_string(),
                    text: "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Tellus rutrum tellus pellentesque eu tincidunt tortor. Aenean et tortor at risus viverra adipiscing. Posuere sollicitudin aliquam ultrices sagittis orci a scelerisque purus semper. Tempus imperdiet nulla malesuada pellentesque elit eget gravida cum. Enim praesent elementum facilisis leo vel. Sit amet est placerat in egestas erat. Nisl purus in mollis nunc sed id. Sit amet consectetur adipiscing elit duis tristique sollicitudin nibh sit. Auctor elit sed vulputate mi. Ullamcorper dignissim cras tincidunt lobortis feugiat vivamus. Aliquet sagittis id consectetur purus. Mauris augue neque gravida in fermentum et sollicitudin. Ultrices eros in cursus turpis massa. Turpis cursus in hac habitasse platea dictumst quisque. Ultricies tristique nulla aliquet enim. Luctus venenatis lectus magna fringilla. Sit amet nulla facilisi morbi tempus.
<br/><br/>
Mauris augue neque gravida in fermentum et sollicitudin. Non blandit massa enim nec dui nunc mattis. Commodo viverra maecenas accumsan lacus vel facilisis volutpat est velit. Felis eget velit aliquet sagittis id consectetur purus ut. Mauris sit amet massa vitae tortor condimentum lacinia. Odio facilisis mauris sit amet massa. Viverra vitae congue eu consequat. Amet purus gravida quis blandit turpis cursus in. Neque viverra justo nec ultrices dui. Quisque non tellus orci ac auctor augue mauris augue neque. Urna duis convallis convallis tellus. Eu volutpat odio facilisis mauris sit amet. Sed risus pretium quam vulputate dignissim suspendisse in est. Porttitor leo a diam sollicitudin. Morbi non arcu risus quis varius quam quisque. Lacus sed viverra tellus in hac habitasse. Diam quam nulla porttitor massa id neque. Tincidunt praesent semper feugiat nibh.
<br/><br/>
Quis eleifend quam adipiscing vitae proin sagittis nisl rhoncus. Tempus imperdiet nulla malesuada pellentesque elit eget gravida. Morbi leo urna molestie at elementum. Neque vitae tempus quam pellentesque. In iaculis nunc sed augue lacus viverra. Nibh cras pulvinar mattis nunc sed blandit libero volutpat sed. Sit amet justo donec enim diam. Dolor sit amet consectetur adipiscing elit ut aliquam purus sit. Id venenatis a condimentum vitae sapien pellentesque habitant. Malesuada fames ac turpis egestas sed tempus urna et. Et tortor consequat id porta nibh venenatis cras. Porttitor lacus luctus accumsan tortor posuere ac ut. Maecenas volutpat blandit aliquam etiam erat velit scelerisque in dictum. Neque viverra justo nec ultrices dui sapien eget mi proin. Convallis tellus id interdum velit laoreet. Curabitur vitae nunc sed velit dignissim sodales ut eu sem. Eget arcu dictum varius duis at.
<br/><br/>
Orci a scelerisque purus semper eget duis at tellus at. Mi ipsum faucibus vitae aliquet nec ullamcorper. Tincidunt id aliquet risus feugiat. Lorem ipsum dolor sit amet consectetur. Sollicitudin ac orci phasellus egestas tellus rutrum tellus. Facilisis volutpat est velit egestas dui id ornare arcu odio. Vehicula ipsum a arcu cursus vitae congue. Ultricies lacus sed turpis tincidunt id aliquet. Id ornare arcu odio ut. Velit sed ullamcorper morbi tincidunt ornare massa eget. Pharetra sit amet aliquam id diam maecenas. Vulputate sapien nec sagittis aliquam malesuada.
<br/><br/>
Vulputate sapien nec sagittis aliquam malesuada bibendum arcu vitae. Sed risus ultricies tristique nulla. Massa vitae tortor condimentum lacinia quis vel eros donec ac. Et malesuada fames ac turpis. Sit amet aliquam id diam maecenas ultricies mi eget. Neque convallis a cras semper auctor neque vitae tempus. At tempor commodo ullamcorper a. Id aliquet risus feugiat in ante metus dictum. Neque viverra justo nec ultrices dui sapien. Pulvinar pellentesque habitant morbi tristique. Porttitor lacus luctus accumsan tortor posuere. Nam at lectus urna duis. In egestas erat imperdiet sed euismod.".to_string(),
                    thumbnail: "/static/img/thumbnail.png".to_string(),
                    published: Some(Utc::now()),
                },
                ArticleModel {
                    id: 2,
                    title: "Hello Planet!".to_string(),
                    author: "Snens".to_string(),
                    description: "Ullamcorper morbi tincidunt ornare massa eget egestas.".to_string(),
                    text: "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Id neque aliquam vestibulum morbi. Adipiscing vitae proin sagittis nisl rhoncus mattis rhoncus urna neque. Nulla facilisi cras fermentum odio eu. In massa tempor nec feugiat nisl. Mattis vulputate enim nulla aliquet porttitor lacus luctus accumsan. Ullamcorper eget nulla facilisi etiam. Iaculis urna id volutpat lacus laoreet non curabitur gravida. Gravida arcu ac tortor dignissim convallis aenean et tortor at. Massa ultricies mi quis hendrerit dolor magna eget. Accumsan lacus vel facilisis volutpat est velit egestas dui. Vitae purus faucibus ornare suspendisse sed. Pulvinar etiam non quam lacus suspendisse faucibus interdum posuere. Blandit libero volutpat sed cras ornare arcu dui. Odio ut sem nulla pharetra diam sit amet. Tortor dignissim convallis aenean et tortor at risus. Netus et malesuada fames ac turpis egestas. Orci sagittis eu volutpat odio facilisis mauris sit amet. Justo nec ultrices dui sapien eget mi.
<br/><br/>
Neque aliquam vestibulum morbi blandit cursus risus. Pellentesque elit eget gravida cum sociis natoque penatibus et magnis. Faucibus in ornare quam viverra orci. Quam adipiscing vitae proin sagittis nisl rhoncus mattis rhoncus. Ut tortor pretium viverra suspendisse. Feugiat scelerisque varius morbi enim. Netus et malesuada fames ac. Imperdiet proin fermentum leo vel orci porta. Ut consequat semper viverra nam libero. Justo nec ultrices dui sapien.
<br/><br/>
Ipsum dolor sit amet consectetur adipiscing elit. Eleifend mi in nulla posuere sollicitudin aliquam ultrices sagittis orci. Tortor vitae purus faucibus ornare suspendisse sed nisi. Sed libero enim sed faucibus turpis in eu mi. Dignissim sodales ut eu sem integer vitae. Gravida in fermentum et sollicitudin ac orci phasellus egestas. Amet porttitor eget dolor morbi non. Malesuada fames ac turpis egestas sed tempus. Iaculis eu non diam phasellus vestibulum. Orci sagittis eu volutpat odio facilisis mauris. Enim nunc faucibus a pellentesque sit amet porttitor eget dolor. Ut lectus arcu bibendum at. Sed faucibus turpis in eu mi bibendum neque. Dictum varius duis at consectetur lorem. A erat nam at lectus urna duis. Interdum varius sit amet mattis vulputate enim.
<br/><br/>
A pellentesque sit amet porttitor eget dolor morbi non. Iaculis urna id volutpat lacus. Placerat orci nulla pellentesque dignissim enim sit amet venenatis urna. Ac placerat vestibulum lectus mauris ultrices eros in cursus. Nunc consequat interdum varius sit amet mattis vulputate enim nulla. Suspendisse sed nisi lacus sed viverra. Pulvinar neque laoreet suspendisse interdum consectetur libero. Ut tristique et egestas quis ipsum suspendisse ultrices gravida dictum. Turpis in eu mi bibendum neque egestas. Consectetur adipiscing elit pellentesque habitant morbi tristique senectus. Ipsum consequat nisl vel pretium. Suspendisse ultrices gravida dictum fusce ut placerat. Aliquam sem fringilla ut morbi tincidunt augue interdum velit euismod. Amet nisl purus in mollis nunc sed id semper risus. Nisi scelerisque eu ultrices vitae auctor. Ac turpis egestas sed tempus urna et pharetra pharetra massa. Tortor dignissim convallis aenean et. Vel turpis nunc eget lorem dolor sed viverra. Lacus viverra vitae congue eu consequat ac felis donec et. Donec enim diam vulputate ut pharetra sit.
<br/><br/>
Sed viverra tellus in hac. Tellus cras adipiscing enim eu turpis egestas pretium. Arcu ac tortor dignissim convallis aenean et tortor. Lobortis feugiat vivamus at augue eget arcu dictum varius duis. Volutpat blandit aliquam etiam erat velit scelerisque in. Neque egestas congue quisque egestas. Scelerisque varius morbi enim nunc faucibus a. Purus in mollis nunc sed. Ultricies mi quis hendrerit dolor magna. Egestas pretium aenean pharetra magna ac placerat vestibulum. Tristique risus nec feugiat in fermentum posuere.".to_string(),
                    thumbnail: "/static/img/thumbnail2.jpeg".to_string(),
                    published: Some(Utc::now()),
                },
                ArticleModel {
                    id: 3,
                    title: "Hello Universe!".to_string(),
                    author: "Wappen".to_string(),
                    description: "Accumsan lacus vel facilisis volutpat est velit egestas. Suspendisse interdum consectetur libero id faucibus.".to_string(),
                    text: "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Consectetur a erat nam at lectus urna duis convallis convallis. Porta non pulvinar neque laoreet. Est ultricies integer quis auctor elit sed vulputate. In est ante in nibh. Pellentesque eu tincidunt tortor aliquam nulla facilisi cras. Habitant morbi tristique senectus et. Arcu dui vivamus arcu felis bibendum ut. Quisque sagittis purus sit amet volutpat consequat mauris nunc congue. Augue ut lectus arcu bibendum at. In fermentum et sollicitudin ac orci phasellus. Vulputate eu scelerisque felis imperdiet proin fermentum leo vel orci. Amet mattis vulputate enim nulla aliquet porttitor lacus luctus. Sed lectus vestibulum mattis ullamcorper velit sed ullamcorper. Ut lectus arcu bibendum at varius vel pharetra vel turpis. Tincidunt tortor aliquam nulla facilisi. In hac habitasse platea dictumst vestibulum.
<br/><br/>
Tortor condimentum lacinia quis vel eros donec ac odio tempor. Ullamcorper velit sed ullamcorper morbi tincidunt ornare massa eget. Bibendum at varius vel pharetra vel turpis nunc eget lorem. Eget mi proin sed libero enim sed faucibus. Est ante in nibh mauris cursus mattis molestie. In egestas erat imperdiet sed euismod nisi. Nulla facilisi etiam dignissim diam quis. At elementum eu facilisis sed odio morbi. Ultrices tincidunt arcu non sodales neque sodales ut etiam sit. Dui nunc mattis enim ut tellus elementum sagittis vitae. Aliquet bibendum enim facilisis gravida neque convallis a cras semper. Feugiat pretium nibh ipsum consequat nisl. Elit ut aliquam purus sit amet luctus venenatis. Posuere morbi leo urna molestie at elementum eu. Enim ut tellus elementum sagittis vitae et.
<br/><br/>
Purus semper eget duis at tellus. Nulla aliquet enim tortor at. Sit amet volutpat consequat mauris nunc congue nisi vitae suscipit. Morbi tempus iaculis urna id volutpat lacus laoreet non. Ultrices dui sapien eget mi proin sed libero enim sed. Mauris a diam maecenas sed enim ut sem viverra. Cras ornare arcu dui vivamus arcu felis bibendum ut tristique. Odio euismod lacinia at quis risus sed vulputate. Sapien nec sagittis aliquam malesuada. Felis imperdiet proin fermentum leo. Nisl nunc mi ipsum faucibus vitae aliquet. Purus faucibus ornare suspendisse sed nisi lacus sed viverra tellus. Vel pretium lectus quam id leo in. Eu sem integer vitae justo eget magna.
<br/><br/>
Odio eu feugiat pretium nibh. Massa tincidunt nunc pulvinar sapien et ligula ullamcorper malesuada proin. Lorem donec massa sapien faucibus et molestie. Mi eget mauris pharetra et ultrices neque. Eget mauris pharetra et ultrices neque. Hac habitasse platea dictumst quisque sagittis purus sit amet. Euismod elementum nisi quis eleifend quam adipiscing vitae proin sagittis. Ultricies lacus sed turpis tincidunt id aliquet risus. Odio morbi quis commodo odio aenean sed adipiscing diam. Pellentesque pulvinar pellentesque habitant morbi tristique senectus et netus. Sed ullamcorper morbi tincidunt ornare massa. Tortor pretium viverra suspendisse potenti. Nascetur ridiculus mus mauris vitae ultricies leo integer.
<br/><br/>
Pellentesque massa placerat duis ultricies lacus sed turpis tincidunt. Lectus vestibulum mattis ullamcorper velit sed ullamcorper morbi tincidunt. Venenatis cras sed felis eget velit aliquet. Consectetur lorem donec massa sapien faucibus et molestie ac feugiat. Morbi blandit cursus risus at ultrices mi. Iaculis nunc sed augue lacus viverra vitae. Posuere sollicitudin aliquam ultrices sagittis orci. Turpis egestas maecenas pharetra convallis posuere. Lacus sed turpis tincidunt id aliquet risus feugiat in. Nibh cras pulvinar mattis nunc sed blandit. Vulputate eu scelerisque felis imperdiet.".to_string(),
                    thumbnail: "/static/img/thumbnail3.png".to_string(),
                    published: None,
                },
            ],
        }
    }
}

impl ArticleRepository for MockArticleRepository {
    fn find_all(&self) -> Vec<ArticleModel> {
        self.articles.clone()
    }

    fn find_by_id(&self, id: i32) -> Option<ArticleModel> {
        self.articles.iter().find(|a| a.id == id).cloned()
    }
}
